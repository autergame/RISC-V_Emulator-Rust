use std::collections::HashMap;

use inst_defs::*;
use instructions::Instruction;

fn str_is_in_list(list: &[&str], str: &str) -> Option<usize> {
    return list.iter().position(|&keyword| keyword == str);
}

fn hex_or_decimal_from_string(str: &str) -> Option<u32> {
    if check_valid_hex_or_decimal(str) == false {
        return None;
    }

    let mut str_len = str.len();
    let mut str_iter = str.chars().peekable();

    let mut negative = false;
    if str_iter.peek() == Some(&'+') {
        str_iter.next();
    } else if str_iter.peek() == Some(&'-') {
        str_iter.next();
        str_len -= 1;
        negative = true;
    }

    let mut result: u32;

    let str_iter_str = str_iter.clone().collect::<String>();
    if (str_len > 2) && (str_iter_str.starts_with("0x") | str_iter_str.starts_with("0X")) {
        result = u32::from_str_radix(&str_iter_str[2..], 16).unwrap();
    } else {
        result = u32::from_str_radix(&str_iter_str, 10).unwrap();
    }

    if negative {
        result = (-(result as i32)) as u32;
    }

    return Some(result);
}

fn check_valid_hex_or_decimal(str: &str) -> bool {
    let mut str_len = str.len();
    if str_len == 0 {
        return false;
    }

    let mut str_iter = str.chars().peekable();

    if str_iter.peek() == Some(&'+') {
        str_iter.next();
    } else if str_iter.peek() == Some(&'-') {
        str_iter.next();
        str_len -= 1;
    }

    let is_valid;

    if str_len >= 1 {
        let str_iter_str = str_iter.clone().collect::<String>();
        if (str_len > 2) && (str_iter_str.starts_with("0x") | str_iter_str.starts_with("0X")) {
            is_valid = str_iter.skip(2).all(|c| c.is_digit(16));
        } else {
            is_valid = str_iter.all(|c| c.is_digit(10));
        }
    } else {
        is_valid = false;
    }

    return is_valid;
}

pub fn assemble(insts: &str) -> Vec<u32> {
    let insts_splitted = insts.lines();

    let insts_cleaned = insts_splitted
        .map(|line| line.split('#').next().unwrap())
        .filter(|line| !line.is_empty());

    let tokens = insts_cleaned
        .flat_map(|line| {
            line.split(&[' ', ',', '\t'])
                .filter(|line| !line.is_empty())
        })
        .collect::<Vec<&str>>();

    let mut label_list: HashMap<&str, usize> = HashMap::new();

    tokens
        .iter()
        .filter(|token| KEYWORDS.contains(token) || token.ends_with(":"))
        .enumerate()
        .filter(|(_, token)| token.ends_with(':'))
        .for_each(|(i, token)| {
            label_list.insert(token.strip_suffix(":").unwrap(), (i - label_list.len()) * 4);
        });

    let mut tokens_list = tokens.into_iter().filter(|token| !token.ends_with(':'));

    let mut compiled_insts: Vec<u32> = Vec::new();

    while let Some(token_1) = tokens_list.next() {
        let opcode = str_is_in_list(KEYWORDS, token_1);
        if opcode == None {
            println!("Skipping unknown opcode: {}", token_1);
            continue;
        }
        let opcode = opcode.unwrap();

        match opcode {
			// U
			0 |    // lui
			1 => { // auipc
				let keyword = KEYWORDS[opcode];
				let token_2 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 2 tokens: rd imm", keyword));
				let token_3 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 1 token: imm", keyword));
				let rd = str_is_in_list(REGISTERS, token_2).unwrap_or_else(|| panic!("Unknown register rd: {}", token_2));
				let imm = hex_or_decimal_from_string(token_3).unwrap_or_else(|| panic!("Invalid number / hex: {}", token_3));
				let inst_funct_type = &OPCODE_FUNCTS[opcode];
				if let InstFnTypes::InstFn2ArgsU32(inst_funct) = inst_funct_type {
					let inst = inst_funct(REGISTERS_INDEX[rd], imm);
					compiled_insts.push(inst.get_bits());
				}
			},
			// J
			2 => { // jal
				let keyword = KEYWORDS[opcode];
				let token_2 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 2 tokens: rd imm/label", keyword));
				let token_3 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 1 token: imm/label", keyword));
				let rd = str_is_in_list(REGISTERS, token_2).unwrap_or_else(|| panic!("Unknown register rd: {}", token_2));
				let imm = get_and_convert_label_from_hashmap(&label_list, token_3, compiled_insts.len());
				let inst_funct_type = &OPCODE_FUNCTS[opcode];
				if let InstFnTypes::InstFn2ArgsI32(inst_funct) = inst_funct_type {
					let inst = inst_funct(REGISTERS_INDEX[rd], imm as i32);
					compiled_insts.push(inst.get_bits());
				}
			}
			// R
			27 | 	// add
			28 | 	// sub
			29 | 	// sll
			30 | 	// slt
			31 | 	// sltu
			32 | 	// xor
			33 | 	// srl
			34 | 	// sra
			35 | 	// or
			36 => { // and
				let keyword = KEYWORDS[opcode];
				let token_2 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 3 tokens: rd rs1 rs2", keyword));
				let token_3 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 2 tokens: rs1 rs2", keyword));
				let token_4 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 1 token: rs2", keyword));
				let rd = str_is_in_list(REGISTERS, token_2).unwrap_or_else(|| panic!("Unknown register rd: {}", token_2));
				let rs1 = str_is_in_list(REGISTERS, token_3).unwrap_or_else(|| panic!("Unknown register rs1: {}", token_3));
				let rs2 = str_is_in_list(REGISTERS, token_4).unwrap_or_else(|| panic!("Unknown register rs2: {}", token_4));
				let inst_funct_type = &OPCODE_FUNCTS[opcode];
				if let InstFnTypes::InstFn3ArgsU32(inst_funct) = inst_funct_type {
					let inst = inst_funct(REGISTERS_INDEX[rd], REGISTERS_INDEX[rs1], REGISTERS_INDEX[rs2]);
					compiled_insts.push(inst.get_bits());
				}
			}
			// I
			3 | 	// jalr
			10 | 	// lb
			11 | 	// lh
			12 | 	// lw
			13 | 	// lbu
			14 | 	// lhu
			18 | 	// addi
			19 | 	// slti
			20 | 	// sltiu
			21 | 	// xori
			22 | 	// ori
			23 | 	// andi
			// Shift
			24 | 	// slli
			25 | 	// srli
			26 => { // srai
				let keyword = KEYWORDS[opcode];
				let token_2 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 3 tokens: rd rs1 imm", keyword));
				let token_3 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 2 tokens: rs1 imm", keyword));
				let token_4 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 1 token: imm", keyword));
				let rd = str_is_in_list(REGISTERS, token_2).unwrap_or_else(|| panic!("Unknown register rd: {}", token_2));
				let rs1 = str_is_in_list(REGISTERS, token_3).unwrap_or_else(|| panic!("Unknown register rs1: {}", token_3));
				let imm = hex_or_decimal_from_string(token_4).unwrap_or_else(|| panic!("Invalid number / hex: {}", token_4));
				let inst_funct_type = &OPCODE_FUNCTS[opcode];
				if let InstFnTypes::InstFn3ArgsI32(inst_funct) = inst_funct_type {
					let inst = inst_funct(REGISTERS_INDEX[rd], REGISTERS_INDEX[rs1], imm as i32);
					compiled_insts.push(inst.get_bits());
				}
			}
			// B
			4 |    // beq
			5 |    // bne
			6 |    // blt
			7 |    // bge
			8 |    // bltu
			9 => { // bgeu
				let keyword = KEYWORDS[opcode];
				let token_2 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 3 tokens: rs1 rs2 imm/label", keyword));
				let token_3 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 2 tokens: rs2 imm/label", keyword));
				let token_4 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 1 token: imm/label", keyword));
				let rs1 = str_is_in_list(REGISTERS, token_2).unwrap_or_else(|| panic!("Unknown register rs1: {}", token_2));
				let rs2 = str_is_in_list(REGISTERS, token_3).unwrap_or_else(|| panic!("Unknown register rs2: {}", token_3));
				let imm = get_and_convert_label_from_hashmap(&label_list, token_4, compiled_insts.len());
				let inst_funct_type = &OPCODE_FUNCTS[opcode];
				if let InstFnTypes::InstFn3ArgsI32(inst_funct) = inst_funct_type {
					let inst = inst_funct(REGISTERS_INDEX[rs1], REGISTERS_INDEX[rs2], imm as i32);
					compiled_insts.push(inst.get_bits());
				}
			}
			// S
			15 | 	// sb
			16 | 	// sh
			17 => { // sw
				let keyword = KEYWORDS[opcode];
				let token_2 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 3 tokens: rs1 rs2 imm", keyword));
				let token_3 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 2 tokens: rs2 imm", keyword));
				let token_4 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 1 token: imm", keyword));
				let rs1 = str_is_in_list(REGISTERS, token_2).unwrap_or_else(|| panic!("Unknown register rs1: {}", token_2));
				let rs2 = str_is_in_list(REGISTERS, token_3).unwrap_or_else(|| panic!("Unknown register rs2: {}", token_3));
				let imm = hex_or_decimal_from_string(token_4).unwrap_or_else(|| panic!("Invalid number / hex: {}", token_4));
				let inst_funct_type = &OPCODE_FUNCTS[opcode];
				if let InstFnTypes::InstFn3ArgsI32(inst_funct) = inst_funct_type {
					let inst = inst_funct(REGISTERS_INDEX[rs1], REGISTERS_INDEX[rs2], imm as i32);
					compiled_insts.push(inst.get_bits());
				}
			}
			// E
			37 | 	// ecall
			38 => { // ebreak
				let inst_funct_type = &OPCODE_FUNCTS[opcode];
				if let InstFnTypes::InstFn0Args(inst_funct) = inst_funct_type {
					let inst = inst_funct();
					compiled_insts.push(inst.get_bits());
				}
			}
			// CSR
			39 | 	// csrrw
			40 | 	// csrrs
			41 => { // csrrc
				let keyword = KEYWORDS[opcode];
				let token_2 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 3 tokens: rd csr rs1", keyword));
				let token_3 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 2 tokens: csr rs1", keyword));
				let token_4 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 1 token: rs1", keyword));
				let rd = str_is_in_list(REGISTERS, token_2).unwrap_or_else(|| panic!("Unknown register rd: {}", token_2));
				let csr = hex_or_decimal_from_string(token_3).unwrap_or_else(|| panic!("Invalid csr number / hex: {}", token_3));
				let rs1 = str_is_in_list(REGISTERS, token_4).unwrap_or_else(|| panic!("Unknown register rs1: {}", token_4));
				let inst_funct_type = &OPCODE_FUNCTS[opcode];
				if let InstFnTypes::InstFn3ArgsI32(inst_funct) = inst_funct_type {
					let inst = inst_funct(REGISTERS_INDEX[rd], REGISTERS_INDEX[rs1], csr as i32);
					compiled_insts.push(inst.get_bits());
				}
			}
			// CSR I
			42 |    // csrrwi
			43 |    // csrrsi
			44 => { // csrrci
				let keyword = KEYWORDS[opcode];
				let token_2 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 3 tokens: rd csr zimm", keyword));
				let token_3 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 2 tokens: csr zimm", keyword));
				let token_4 = tokens_list.next().unwrap_or_else(|| panic!("Opcode: {} need more 1 token: zimm", keyword));
				let rd = str_is_in_list(REGISTERS, token_2).unwrap_or_else(|| panic!("Unknown register rd: {}", token_2));
				let csr = hex_or_decimal_from_string(token_3).unwrap_or_else(|| panic!("Invalid csr number / hex: {}", token_3));
				let zimm = hex_or_decimal_from_string(token_4).unwrap_or_else(|| panic!("Invalid zimm number / hex: {}", token_4));
				let inst_funct_type = &OPCODE_FUNCTS[opcode];
				if let InstFnTypes::InstFn3ArgsI32(inst_funct) = inst_funct_type {
					let inst = inst_funct(REGISTERS_INDEX[rd], zimm, csr as i32);
					compiled_insts.push(inst.get_bits());
				}
			}
			_ => {}
		}
    }

    return compiled_insts;
}

fn get_and_convert_label_from_hashmap(
    label_list: &HashMap<&str, usize>,
    label: &str,
    last_pointer: usize,
) -> u32 {
    let mut imm: u32;
    if let Some(label_value) = label_list.get(label) {
        imm = *label_value as u32;
        let insts_pointer = (last_pointer * 4) as u32;
        if imm < insts_pointer {
            imm = (-((insts_pointer - imm) as i32)) as u32;
        } else {
            imm -= insts_pointer;
        }
    } else {
        imm = hex_or_decimal_from_string(label)
            .unwrap_or_else(|| panic!("Unknown label or invalid number / hex: {}", label));
    }
    return imm;
}

const KEYWORDS: &[&str; 45] = &[
    "lui", "auipc", "jal", "jalr", "beq", "bne", "blt", "bge", "bltu", "bgeu", "lb", "lh", "lw",
    "lbu", "lhu", "sb", "sh", "sw", "addi", "slti", "sltiu", "xori", "ori", "andi", "slli", "srli",
    "srai", "add", "sub", "sll", "slt", "sltu", "xor", "srl", "sra", "or", "and", "ecall",
    "ebreak", "csrrw", "csrrs", "csrrc", "csrrwi", "csrrsi", "csrrci",
];

enum InstFnTypes {
    InstFn0Args(fn() -> Instruction),
    InstFn2ArgsI32(fn(u32, i32) -> Instruction),
    InstFn2ArgsU32(fn(u32, u32) -> Instruction),
    InstFn3ArgsI32(fn(u32, u32, i32) -> Instruction),
    InstFn3ArgsU32(fn(u32, u32, u32) -> Instruction),
}

use self::InstFnTypes::*;

const OPCODE_FUNCTS: &[InstFnTypes; 45] = &[
    InstFn2ArgsU32(inst_lui),
    InstFn2ArgsU32(inst_auipc),
    InstFn2ArgsI32(inst_jal),
    InstFn3ArgsI32(inst_jalr),
    InstFn3ArgsI32(inst_beq),
    InstFn3ArgsI32(inst_bne),
    InstFn3ArgsI32(inst_blt),
    InstFn3ArgsI32(inst_bge),
    InstFn3ArgsI32(inst_bltu),
    InstFn3ArgsI32(inst_bgeu),
    InstFn3ArgsI32(inst_lb),
    InstFn3ArgsI32(inst_lh),
    InstFn3ArgsI32(inst_lw),
    InstFn3ArgsI32(inst_lbu),
    InstFn3ArgsI32(inst_lhu),
    InstFn3ArgsI32(inst_sb),
    InstFn3ArgsI32(inst_sh),
    InstFn3ArgsI32(inst_sw),
    InstFn3ArgsI32(inst_addi),
    InstFn3ArgsI32(inst_slti),
    InstFn3ArgsI32(inst_sltiu),
    InstFn3ArgsI32(inst_xori),
    InstFn3ArgsI32(inst_ori),
    InstFn3ArgsI32(inst_andi),
    InstFn3ArgsU32(inst_slli),
    InstFn3ArgsU32(inst_srli),
    InstFn3ArgsU32(inst_srai),
    InstFn3ArgsU32(inst_add),
    InstFn3ArgsU32(inst_sub),
    InstFn3ArgsU32(inst_sll),
    InstFn3ArgsU32(inst_slt),
    InstFn3ArgsU32(inst_sltu),
    InstFn3ArgsU32(inst_xor),
    InstFn3ArgsU32(inst_srl),
    InstFn3ArgsU32(inst_sra),
    InstFn3ArgsU32(inst_or),
    InstFn3ArgsU32(inst_and),
    InstFn0Args(inst_ecall),
    InstFn0Args(inst_ebreak),
    InstFn3ArgsI32(inst_csrrw),
    InstFn3ArgsI32(inst_csrrs),
    InstFn3ArgsI32(inst_csrrc),
    InstFn3ArgsI32(inst_csrrwi),
    InstFn3ArgsI32(inst_csrrsi),
    InstFn3ArgsI32(inst_csrrci),
];

const REGISTERS: &[&str; 65] = &[
    "zero", "ra", "sp", "gp", "tp", "fp", "a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "s0",
    "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t0", "t1", "t2", "t3",
    "t4", "t5", "t6", "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "x10", "x11",
    "x12", "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24",
    "x25", "x26", "x27", "x28", "x29", "x30", "x31",
];

const REGISTERS_INDEX: &[u32; 65] = &[
    0, 1, 2, 3, 4, 8, 10, 11, 12, 13, 14, 15, 16, 17, 8, 9, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    5, 6, 7, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
];
