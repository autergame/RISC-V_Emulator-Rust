use inst_defs::*;
use instructions::Instruction;

fn str_is_in_list(list: &[&str], str: &str) -> Option<usize> {
    return list.iter().position(|&keyword| keyword == str);
}

fn hex_or_decimal_from_string(str: &str) -> u32 {
    let mut str_len = str.len();
    if str_len == 0 {
        return 0;
    }

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

    if str_len >= 1 {
        let str_iter_str = str_iter.clone().collect::<String>();
        if (str_len > 2) && (str_iter_str.starts_with("0x") | str_iter_str.starts_with("0X")) {
            result = u32::from_str_radix(&str_iter_str[2..], 16).unwrap();
        } else {
            result = u32::from_str_radix(str_iter_str.as_str(), 10).unwrap();
        }
        if negative {
            result = (-(result as i32)) as u32;
        }
    } else {
        result = 0;
    }

    return result;
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

    let mut label_list: Vec<(&str, usize)> = Vec::new();

    tokens
        .iter()
        .filter(|token| KEYWORDS.contains(token) || token.ends_with(":"))
        .enumerate()
        .filter(|(_, token)| token.ends_with(':'))
        .for_each(|(i, token)| {
            label_list.push((token.strip_suffix(":").unwrap(), (i - label_list.len()) * 4))
        });

    let tokens_list = tokens
        .into_iter()
        .filter(|token| !token.ends_with(':'))
        .enumerate()
        .collect::<Vec<(usize, &str)>>();

    let tokens_count = tokens_list.len();
    let mut tokens_list_iter = tokens_list.into_iter();

    let mut compiled_insts: Vec<u32> = Vec::new();

    while let Some((i, token_1)) = tokens_list_iter.next() {
        let opcode = str_is_in_list(KEYWORDS, token_1);

        match opcode {
			// U
			Some(0) |    // lui
			Some(1) => { // auipc
				if (i + 2) > tokens_count {
					println!("Opcode: {} need more 2 tokens: rd imm", KEYWORDS[opcode.unwrap()]);
					break;
				}
				if (i + 1) > tokens_count {
					println!("Opcode: {} need more 1 token: imm", KEYWORDS[opcode.unwrap()]);
					break;
				}
				let token_2 = tokens_list_iter.next().unwrap().1;
				let rd = str_is_in_list(REGISTERS, token_2);
				if rd == None {
					println!("Unknown register rd: {}", token_2);
					break;
				}
				let token_3 = tokens_list_iter.next().unwrap().1;
				if check_valid_hex_or_decimal(token_3) == false {
					println!("Invalid number / hex: {}", token_3);
					break;
				}
				let imm = hex_or_decimal_from_string(token_3);
				let inst_funct_type = &OPCODE_FUNCTS[opcode.unwrap()];
				if let InstFnTypes::InstFn2ArgsU32(inst_funct) = inst_funct_type {
					let inst = inst_funct(REGISTERS_INDEX[rd.unwrap()], imm);
					compiled_insts.push(inst.get_bits());
				}
			},
			// J
			Some(2) => { // jal
				if (i + 2) > tokens_count {
					println!("Opcode: {} need more 2 tokens: rd imm/label", KEYWORDS[opcode.unwrap()]);
					break;
				}
				if (i + 1) > tokens_count {
					println!("Opcode: {} need more 1 token: imm/label", KEYWORDS[opcode.unwrap()]);
					break;
				}
				let token_2 = tokens_list_iter.next().unwrap().1;
				let rd = str_is_in_list(REGISTERS, token_2);
				if rd == None {
					println!("Unknown register rd: {}", token_2);
					break;
				}
				let token_3 = tokens_list_iter.next().unwrap().1;
				let mut imm: u32;
				if let Some(label_index) = label_list.iter().position(|&(label, _)| label == token_3) {
					imm = label_list[label_index].1 as u32;
					let insts_pointer = (compiled_insts.len() * 4) as u32;
					if imm < insts_pointer {
						imm = (-((insts_pointer - imm) as i32)) as u32;
					} else {
						imm -= insts_pointer;
					}
				} else {
					if check_valid_hex_or_decimal(token_3) == false
					{
						println!("Unknown label or invalid number / hex: {}", token_3);
						break;
					}
					imm = hex_or_decimal_from_string(token_3);
				}
				let inst_funct_type = &OPCODE_FUNCTS[opcode.unwrap()];
				if let InstFnTypes::InstFn2ArgsI32(inst_funct) = inst_funct_type {
					let inst = inst_funct(REGISTERS_INDEX[rd.unwrap()], imm as i32);
					compiled_insts.push(inst.get_bits());
				}
			}
			// R
			Some(27) | 	  // add
			Some(28) | 	  // sub
			Some(29) | 	  // sll
			Some(30) | 	  // slt
			Some(31) | 	  // sltu
			Some(32) | 	  // xor
			Some(33) | 	  // srl
			Some(34) | 	  // sra
			Some(35) | 	  // or
			Some(36) => { // and
				if (i + 3) > tokens_count {
					println!("Opcode: {} need more 3 tokens: rd rs1 rs2", KEYWORDS[opcode.unwrap()]);
					break;
				}
				if (i + 2) > tokens_count {
					println!("Opcode: {} need more 2 tokens: rs1 rs2", KEYWORDS[opcode.unwrap()]);
					break;
				}
				if (i + 1) > tokens_count {
					println!("Opcode: {} need more 1 token: rs2", KEYWORDS[opcode.unwrap()]);
					break;
				}
				let token_2 = tokens_list_iter.next().unwrap().1;
				let rd = str_is_in_list(REGISTERS, token_2);
				if rd == None {
					println!("Unknown register rd: {}", token_2);
					break;
				}
				let token_3 = tokens_list_iter.next().unwrap().1;
				let rs1 = str_is_in_list(REGISTERS, token_3);
				if rs1 == None {
					println!("Unknown register rs1: {}", token_3);
					break;
				}
				let token_4 = tokens_list_iter.next().unwrap().1;
				let rs2 = str_is_in_list(REGISTERS, token_4);
				if rs2 == None {
					println!("Unknown register rs2: {}", token_4);
					break;
				}
				let inst_funct_type = &OPCODE_FUNCTS[opcode.unwrap()];
				if let InstFnTypes::InstFn3ArgsU32(inst_funct) = inst_funct_type {
					let inst = inst_funct(REGISTERS_INDEX[rd.unwrap()],
						REGISTERS_INDEX[rs1.unwrap()],  REGISTERS_INDEX[rs2.unwrap()]);
					compiled_insts.push(inst.get_bits());
				}
			}
			// I
			Some(3) | 	  // jalr
			Some(10) | 	  // lb
			Some(11) | 	  // lh
			Some(12) | 	  // lw
			Some(13) | 	  // lbu
			Some(14) | 	  // lhu
			Some(18) | 	  // addi
			Some(19) | 	  // slti
			Some(20) | 	  // sltiu
			Some(21) | 	  // xori
			Some(22) | 	  // ori
			Some(23) | 	  // andi
			// Shift
			Some(24) | 	  // slli
			Some(25) | 	  // srli
			Some(26) => { // srai
				if (i + 3) > tokens_count {
					println!("Opcode: {} need more 3 tokens: rd rs1 imm", KEYWORDS[opcode.unwrap()]);
					break;
				}
				if (i + 2) > tokens_count {
					println!("Opcode: {} need more 2 tokens: rs1 imm", KEYWORDS[opcode.unwrap()]);
					break;
				}
				if (i + 1) > tokens_count {
					println!("Opcode: {} need more 1 token: imm", KEYWORDS[opcode.unwrap()]);
					break;
				}
				let token_2 = tokens_list_iter.next().unwrap().1;
				let rd = str_is_in_list(REGISTERS, token_2);
				if rd == None {
					println!("Unknown register rd: {}", token_2);
					break;
				}
				let token_3 = tokens_list_iter.next().unwrap().1;
				let rs1 = str_is_in_list(REGISTERS, token_3);
				if rs1 == None {
					println!("Unknown register rs1: {}", token_3);
					break;
				}
				let token_4 = tokens_list_iter.next().unwrap().1;
				if check_valid_hex_or_decimal(token_4) == false {
					println!("Invalid number / hex: {}", token_4);
					break;
				}
				let imm = hex_or_decimal_from_string(token_4);
				let inst_funct_type = &OPCODE_FUNCTS[opcode.unwrap()];
				if let InstFnTypes::InstFn3ArgsI32(inst_funct) = inst_funct_type {
					let inst = inst_funct(REGISTERS_INDEX[rd.unwrap()],
						REGISTERS_INDEX[rs1.unwrap()], imm as i32);
					compiled_insts.push(inst.get_bits());
				}
			}
			// B
			Some(4) |    // beq
			Some(5) |    // bne
			Some(6) |    // blt
			Some(7) |    // bge
			Some(8) |    // bltu
			Some(9) => { // bgeu
				if (i + 3) > tokens_count {
					println!("Opcode: {} need more 3 tokens: rs1 rs2 imm/label", KEYWORDS[opcode.unwrap()]);
					break;
				}
				if (i + 2) > tokens_count {
					println!("Opcode: {} need more 2 tokens: rs2 imm/label", KEYWORDS[opcode.unwrap()]);
					break;
				}
				if (i + 1) > tokens_count {
					println!("Opcode: {} need more 1 token: imm/label", KEYWORDS[opcode.unwrap()]);
					break;
				}
				let token_2 = tokens_list_iter.next().unwrap().1;
				let rs1 = str_is_in_list(REGISTERS, token_2);
				if rs1 == None {
					println!("Unknown register rs1: {}", token_2);
					break;
				}
				let token_3 = tokens_list_iter.next().unwrap().1;
				let rs2 = str_is_in_list(REGISTERS, token_3);
				if rs2 == None {
					println!("Unknown register rs2: {}", token_3);
					break;
				}
				let token_4 = tokens_list_iter.next().unwrap().1;
				let mut imm: u32;
				if let Some(label_index) = label_list.iter().position(|&(label, _)| label == token_4) {
					imm = label_list[label_index].1 as u32;
					let insts_pointer = (compiled_insts.len() * 4) as u32;
					if imm < insts_pointer {
						imm = (-((insts_pointer - imm) as i32)) as u32;
					} else {
						imm -= insts_pointer;
					}
				} else {
					if check_valid_hex_or_decimal(token_4) == false
					{
						println!("Unknown label or invalid number / hex: {}", token_4);
						break;
					}
					imm = hex_or_decimal_from_string(token_4);
				}
				let inst_funct_type = &OPCODE_FUNCTS[opcode.unwrap()];
				if let InstFnTypes::InstFn3ArgsI32(inst_funct) = inst_funct_type {
					let inst = inst_funct(REGISTERS_INDEX[rs1.unwrap()],
						REGISTERS_INDEX[rs2.unwrap()], imm as i32);
					compiled_insts.push(inst.get_bits());
				}
			}
			// S
			Some(15) | 	  // sb
			Some(16) | 	  // sh
			Some(17) => { // sw
				if (i + 3) > tokens_count {
					println!("Opcode: {} need more 3 tokens: rs1 rs2 imm", KEYWORDS[opcode.unwrap()]);
					break;
				}
				if (i + 2) > tokens_count {
					println!("Opcode: {} need more 2 tokens: rs2 imm", KEYWORDS[opcode.unwrap()]);
					break;
				}
				if (i + 1) > tokens_count {
					println!("Opcode: {} need more 1 token: imm", KEYWORDS[opcode.unwrap()]);
					break;
				}
				let token_2 = tokens_list_iter.next().unwrap().1;
				let rs1 = str_is_in_list(REGISTERS, token_2);
				if rs1 == None {
					println!("Unknown register rs1: {}", token_2);
					break;
				}
				let token_3 = tokens_list_iter.next().unwrap().1;
				let rs2 = str_is_in_list(REGISTERS, token_3);
				if rs2 == None {
					println!("Unknown register rs2: {}", token_3);
					break;
				}
				let token_4 = tokens_list_iter.next().unwrap().1;
				if check_valid_hex_or_decimal(token_4) == false {
					println!("Invalid number / hex: {}", token_4);
					break;
				}
				let imm = hex_or_decimal_from_string(token_4);
				let inst_funct_type = &OPCODE_FUNCTS[opcode.unwrap()];
				if let InstFnTypes::InstFn3ArgsI32(inst_funct) = inst_funct_type {
					let inst = inst_funct(REGISTERS_INDEX[rs1.unwrap()],
						REGISTERS_INDEX[rs2.unwrap()], imm as i32);
					compiled_insts.push(inst.get_bits());
				}
			}
			// E
			Some(37) | 	  // ecall
			Some(38) => { // ebreak
				let inst_funct_type = &OPCODE_FUNCTS[opcode.unwrap()];
				if let InstFnTypes::InstFn0Args(inst_funct) = inst_funct_type {
					let inst = inst_funct();
					compiled_insts.push(inst.get_bits());
				}
			}
			None | _ => {
				println!("Skipping unknown opcode: {}", token_1);
				continue;
			}
		}
    }

    return compiled_insts;
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
