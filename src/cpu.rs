use std::convert::TryInto;

use imm_enc_dec::sign_extend;
use instructions;
use types::*;

const MEMORY_SIZE: usize = 0xfffff;
const STACK_POINTER: u32 = 0xf8000;

pub struct RiscvCpu {
    pub csrs: Vec<u64>,
    pub memory: Vec<u8>,
    pub registers: Vec<u32>,
    pub program_counter: u32,
}

impl RiscvCpu {
    pub fn new() -> RiscvCpu {
        RiscvCpu {
            program_counter: 0,
            csrs: vec![0u64; 4096],
            registers: vec![0u32; 32],
            memory: vec![0u8; MEMORY_SIZE],
        }
    }
    pub fn reset(&mut self) {
        self.program_counter = 0;
        self.csrs.iter_mut().for_each(|csr| *csr = 0);
        self.memory.iter_mut().for_each(|mem| *mem = 0);
        self.registers.iter_mut().for_each(|reg| *reg = 0);
        self.set_register(2, STACK_POINTER);
    }
    pub fn dump_registers(&self) {
        println!("\npc: {0:>10} 0x{0:08X}", self.program_counter);
        self.registers.iter().enumerate().for_each(|(i, reg)| {
            println!(
                "x{0:<2} {1:>5}: 0x{2:08X} {2:>10} {3:>11}",
                i, REGISTERS_DUMP[i], reg, *reg as i32
            );
        });
    }
    fn write_u16_memory(&mut self, address: usize, value: u16) {
        if let Some(access) = self.memory.get_mut(address..address + 2) {
            access.copy_from_slice(&value.to_le_bytes());
        } else {
            panic!("Memory overflow");
        }
    }
    fn read_u16_memory(&self, address: usize) -> u16 {
        if let Some(access) = self.memory.get(address..address + 2) {
            u16::from_le_bytes(access.try_into().unwrap())
        } else {
            panic!("Memory access out of bounds");
        }
    }
    fn write_u32_memory(&mut self, address: usize, value: u32) {
        if let Some(access) = self.memory.get_mut(address..address + 4) {
            access.copy_from_slice(&value.to_le_bytes());
        } else {
            panic!("Memory overflow");
        }
    }
    fn read_u32_memory(&self, address: usize) -> u32 {
        if let Some(access) = self.memory.get(address..address + 4) {
            u32::from_le_bytes(access.try_into().unwrap())
        } else {
            panic!("Memory access out of bounds");
        }
    }
    pub fn load_from_u8(&mut self, inst_list: &[u8]) {
        self.reset();
        self.memory[..inst_list.len()].copy_from_slice(inst_list);
        self.write_u32_memory(inst_list.len(), 0xDEADC0DE);
    }
    pub fn get_register(&self, index: u32) -> u32 {
        self.registers[index as usize]
    }
    fn set_register(&mut self, index: u32, value: u32) {
        self.registers[index as usize] = value;
    }
    fn execute_opcode_branch(&mut self, bits: u32) {
        let inst_b = instructions::_B(bits);
        let funct3 = inst_b.funct3();
        match funct3 {
            FUNCT3_000 => {
                // beq
                if (self.get_register(inst_b.rs1()) as i32)
                    == (self.get_register(inst_b.rs2()) as i32)
                {
                    self.program_counter =
                        self.program_counter.wrapping_add(inst_b.imm_dec() as u32);
                } else {
                    self.program_counter += 4;
                }
            }
            FUNCT3_001 => {
                // bne
                if (self.get_register(inst_b.rs1()) as i32)
                    != (self.get_register(inst_b.rs2()) as i32)
                {
                    self.program_counter =
                        self.program_counter.wrapping_add(inst_b.imm_dec() as u32);
                } else {
                    self.program_counter += 4;
                }
            }
            FUNCT3_100 => {
                // blt
                if (self.get_register(inst_b.rs1()) as i32)
                    < (self.get_register(inst_b.rs2()) as i32)
                {
                    self.program_counter =
                        self.program_counter.wrapping_add(inst_b.imm_dec() as u32);
                } else {
                    self.program_counter += 4;
                }
            }
            FUNCT3_101 => {
                // bge
                if (self.get_register(inst_b.rs1()) as i32)
                    >= (self.get_register(inst_b.rs2()) as i32)
                {
                    self.program_counter =
                        self.program_counter.wrapping_add(inst_b.imm_dec() as u32);
                } else {
                    self.program_counter += 4;
                }
            }
            FUNCT3_110 => {
                // bltu
                if self.get_register(inst_b.rs1()) < self.get_register(inst_b.rs2()) {
                    self.program_counter =
                        self.program_counter.wrapping_add(inst_b.imm_dec() as u32);
                } else {
                    self.program_counter += 4;
                }
            }
            FUNCT3_111 => {
                // bgeu
                if self.get_register(inst_b.rs1()) >= self.get_register(inst_b.rs2()) {
                    self.program_counter =
                        self.program_counter.wrapping_add(inst_b.imm_dec() as u32);
                } else {
                    self.program_counter += 4;
                }
            }
            _ => panic!("Unimplemented B funct3 {}", funct3),
        }
    }
    fn execute_opcode_load(&mut self, bits: u32) {
        let inst_i = instructions::_I(bits);
        let funct3 = inst_i.funct3();
        match funct3 {
            FUNCT3_000 => {
                // lb
                let address = self
                    .get_register(inst_i.rs1())
                    .wrapping_add(inst_i.imm_dec() as u32);
                let value = self.memory[address as usize];
                self.set_register(inst_i.rd(), sign_extend(value as u32, 8) as u32);
                self.program_counter += 4;
            }
            FUNCT3_001 => {
                // lh
                let address = self
                    .get_register(inst_i.rs1())
                    .wrapping_add(inst_i.imm_dec() as u32);
                let value = self.read_u16_memory(address as usize);
                self.set_register(inst_i.rd(), sign_extend(value as u32, 16) as u32);
                self.program_counter += 4;
            }
            FUNCT3_010 => {
                // lw
                let address = self
                    .get_register(inst_i.rs1())
                    .wrapping_add(inst_i.imm_dec() as u32);
                self.set_register(inst_i.rd(), self.read_u32_memory(address as usize));
                self.program_counter += 4;
            }
            FUNCT3_100 => {
                // lbu
                let address = self
                    .get_register(inst_i.rs1())
                    .wrapping_add(inst_i.imm_dec() as u32);
                self.set_register(inst_i.rd(), self.memory[address as usize] as u32);
                self.program_counter += 4;
            }
            FUNCT3_101 => {
                // lhu
                let address = self
                    .get_register(inst_i.rs1())
                    .wrapping_add(inst_i.imm_dec() as u32);
                self.set_register(inst_i.rd(), self.read_u16_memory(address as usize) as u32);
                self.program_counter += 4;
            }
            _ => panic!("Unimplemented I funct3 {}", funct3),
        }
    }
    fn execute_opcode_store(&mut self, bits: u32) {
        let inst_s = instructions::_S(bits);
        let funct3 = inst_s.funct3();
        match funct3 {
            FUNCT3_000 => {
                // sb
                let address = self
                    .get_register(inst_s.rs1())
                    .wrapping_add(inst_s.imm_dec() as u32);
                self.memory[address as usize] = self.get_register(inst_s.rs2()) as u8;
                self.program_counter += 4;
            }
            FUNCT3_001 => {
                // sh
                let address = self
                    .get_register(inst_s.rs1())
                    .wrapping_add(inst_s.imm_dec() as u32);
                self.write_u16_memory(address as usize, self.get_register(inst_s.rs2()) as u16);
                self.program_counter += 4;
            }
            FUNCT3_010 => {
                // sw
                let address = self
                    .get_register(inst_s.rs1())
                    .wrapping_add(inst_s.imm_dec() as u32);
                self.write_u32_memory(address as usize, self.get_register(inst_s.rs2()));
                self.program_counter += 4;
            }
            _ => panic!("Unimplemented S funct3 {}", funct3),
        }
    }
    fn execute_opcode_alu_and_shift_imm(&mut self, bits: u32) {
        let inst_i = instructions::_I(bits);
        let funct3 = inst_i.funct3();
        match funct3 {
            FUNCT3_000 => {
                // addi
                self.set_register(
                    inst_i.rd(),
                    self.get_register(inst_i.rs1())
                        .wrapping_add(inst_i.imm_dec() as u32),
                );
                self.program_counter += 4;
            }
            FUNCT3_001 => {
                // slli
                let inst_shift = instructions::_Shift(bits);
                self.set_register(
                    inst_shift.rd(),
                    self.get_register(inst_shift.rs1())
                        .wrapping_shl(inst_shift.imm_dec()),
                );
                self.program_counter += 4;
            }
            FUNCT3_010 => {
                // slti
                if (self.get_register(inst_i.rs1()) as i32) < inst_i.imm_dec() {
                    self.set_register(inst_i.rd(), 1);
                } else {
                    self.set_register(inst_i.rd(), 0);
                }
                self.program_counter += 4;
            }
            FUNCT3_011 => {
                // sltiu
                if self.get_register(inst_i.rs1()) < inst_i.imm11_0() {
                    self.set_register(inst_i.rd(), 1);
                } else {
                    self.set_register(inst_i.rd(), 0);
                }
                self.program_counter += 4;
            }
            FUNCT3_100 => {
                // xori
                self.set_register(
                    inst_i.rd(),
                    self.get_register(inst_i.rs1()) ^ (inst_i.imm_dec() as u32),
                );
                self.program_counter += 4;
            }
            FUNCT3_101 => {
                // srli srai
                let inst_shift = instructions::_Shift(bits);
                let funct7 = inst_shift.funct7();
                match funct7 {
                    FUNCT7_0000000 => {
                        // srli
                        self.set_register(
                            inst_shift.rd(),
                            self.get_register(inst_shift.rs1())
                                .wrapping_shr(inst_shift.imm_dec()),
                        );
                        self.program_counter += 4;
                    }
                    FUNCT7_0100000 => {
                        // srai
                        self.set_register(
                            inst_shift.rd(),
                            (self.get_register(inst_shift.rs1()) as i32)
                                .wrapping_shr(inst_shift.imm_dec())
                                as u32,
                        );
                        self.program_counter += 4;
                    }
                    _ => panic!("Unimplemented Shift 101 funct7 {}", funct7),
                }
            }
            FUNCT3_110 => {
                // ori
                self.set_register(
                    inst_i.rd(),
                    self.get_register(inst_i.rs1()) | inst_i.imm_dec() as u32,
                );
                self.program_counter += 4;
            }
            FUNCT3_111 => {
                // andi
                self.set_register(
                    inst_i.rd(),
                    self.get_register(inst_i.rs1()) & inst_i.imm_dec() as u32,
                );
                self.program_counter += 4;
            }
            _ => panic!("Unimplemented I funct3 {}", funct3),
        }
    }
    fn execute_opcode_alu_register(&mut self, bits: u32) {
        let inst_r = instructions::_R(bits);
        let funct3 = inst_r.funct3();
        match funct3 {
            FUNCT3_000 => {
                // add sub
                let funct7 = inst_r.funct7();
                match funct7 {
                    FUNCT7_0000000 => {
                        // add
                        self.set_register(
                            inst_r.rd(),
                            self.get_register(inst_r.rs1())
                                .wrapping_add(self.get_register(inst_r.rs2())),
                        );
                        self.program_counter += 4;
                    }
                    FUNCT7_0100000 => {
                        // sub
                        self.set_register(
                            inst_r.rd(),
                            self.get_register(inst_r.rs1())
                                .wrapping_sub(self.get_register(inst_r.rs2())),
                        );
                        self.program_counter += 4;
                    }
                    _ => panic!("Unimplemented R 000 funct7 {}", funct7),
                }
            }
            FUNCT3_001 => {
                // sll
                self.set_register(
                    inst_r.rd(),
                    self.get_register(inst_r.rs1())
                        .wrapping_shl(self.get_register(inst_r.rs2())),
                );
                self.program_counter += 4;
            }
            FUNCT3_010 => {
                // slt
                if (self.get_register(inst_r.rs1()) as i32)
                    < (self.get_register(inst_r.rs2()) as i32)
                {
                    self.set_register(inst_r.rd(), 1);
                } else {
                    self.set_register(inst_r.rd(), 0);
                }
                self.program_counter += 4;
            }
            FUNCT3_011 => {
                // sltu
                if self.get_register(inst_r.rs1()) < self.get_register(inst_r.rs2()) {
                    self.set_register(inst_r.rd(), 1);
                } else {
                    self.set_register(inst_r.rd(), 0);
                }
                self.program_counter += 4;
            }
            FUNCT3_100 => {
                // xor
                self.set_register(
                    inst_r.rd(),
                    self.get_register(inst_r.rs1()) ^ self.get_register(inst_r.rs2()),
                );
                self.program_counter += 4;
            }
            FUNCT3_101 => {
                // srl sra
                let funct7 = inst_r.funct7();
                match funct7 {
                    FUNCT7_0000000 => {
                        // srl
                        self.set_register(
                            inst_r.rd(),
                            self.get_register(inst_r.rs1())
                                .wrapping_shr(self.get_register(inst_r.rs2())),
                        );
                        self.program_counter += 4;
                    }
                    FUNCT7_0100000 => {
                        // sra
                        self.set_register(
                            inst_r.rd(),
                            (self.get_register(inst_r.rs1()) as i32)
                                .wrapping_shr(self.get_register(inst_r.rs2()))
                                as u32,
                        );
                        self.program_counter += 4;
                    }
                    _ => panic!("Unimplemented R 101 funct7 {}", funct7),
                }
            }
            FUNCT3_110 => {
                // or
                self.set_register(
                    inst_r.rd(),
                    self.get_register(inst_r.rs1()) | self.get_register(inst_r.rs2()),
                );
                self.program_counter += 4;
            }
            FUNCT3_111 => {
                // and
                self.set_register(
                    inst_r.rd(),
                    self.get_register(inst_r.rs1()) & self.get_register(inst_r.rs2()),
                );
                self.program_counter += 4;
            }
            _ => panic!("Unimplemented R funct3 {}", funct3),
        }
    }
    fn execute_opcode_e_and_system(&mut self, bits: u32) {
        let inst_i = instructions::_I(bits);
        let funct3 = inst_i.funct3();
        match funct3 {
            FUNCT3_000 => {
                // ecall ebrak
                let imm11_0 = inst_i.imm11_0();
                match imm11_0 {
                    IMM11_0_000000000000 => {
                        // ecall
                        self.program_counter += 4;
                    }
                    IMM11_0_000000000001 => {
                        // ebreak
                        // #ifdef DEBUG
                        //     __debugbreak();
                        // #endif
                        self.dump_registers();
                        self.program_counter += 4;
                    }
                    _ => panic!("Unimplemented E 000 imm11_0 {}", imm11_0),
                }
            }
            FUNCT3_001 => {
                // csrrw
                let oldcsr = self.csrs[inst_i.imm11_0() as usize];
                self.csrs[inst_i.imm11_0() as usize] = self.get_register(inst_i.rs1()) as u64;
                self.set_register(inst_i.rd(), oldcsr as u32);
                self.program_counter += 4;
            }
            FUNCT3_010 => {
                // csrrs
                let mask = self.get_register(inst_i.rs1());
                let oldcsr = self.csrs[inst_i.imm11_0() as usize];
                self.csrs[inst_i.imm11_0() as usize] = oldcsr | mask as u64;
                self.set_register(inst_i.rd(), oldcsr as u32);
                self.program_counter += 4;
            }
            FUNCT3_011 => {
                // csrrc
                let mask = self.get_register(inst_i.rs1());
                let oldcsr = self.csrs[inst_i.imm11_0() as usize];
                self.csrs[inst_i.imm11_0() as usize] = oldcsr & !(mask as u64);
                self.set_register(inst_i.rd(), oldcsr as u32);
                self.program_counter += 4;
            }
            FUNCT3_101 => {
                // csrrwi
                self.set_register(inst_i.rd(), self.csrs[inst_i.imm11_0() as usize] as u32);
                self.csrs[inst_i.imm11_0() as usize] = inst_i.rs1() as u64;
                self.program_counter += 4;
            }
            FUNCT3_110 => {
                // csrrsi
                let oldcsr = self.csrs[inst_i.imm11_0() as usize];
                self.csrs[inst_i.imm11_0() as usize] = oldcsr | inst_i.rs1() as u64;
                self.set_register(inst_i.rd(), oldcsr as u32);
                self.program_counter += 4;
            }
            FUNCT3_111 => {
                // csrrci
                let oldcsr = self.csrs[inst_i.imm11_0() as usize];
                self.csrs[inst_i.imm11_0() as usize] = oldcsr & !(inst_i.rs1() as u64);
                self.set_register(inst_i.rd(), oldcsr as u32);
                self.program_counter += 4;
            }
            _ => panic!("Unimplemented E funct3 {}", funct3),
        }
    }
    pub fn execute_inst(&mut self, bits: u32) {
        let opcode = instructions::_Op(bits).opcode();
        match opcode {
            OPCODE_LUI => {
                let inst_u = instructions::_U(bits);
                self.set_register(inst_u.rd(), inst_u.imm_dec());
                self.program_counter += 4;
            }
            OPCODE_AUIPC => {
                let inst_u = instructions::_U(bits);
                self.set_register(
                    inst_u.rd(),
                    self.program_counter.wrapping_add(inst_u.imm_dec()),
                );
                self.program_counter += 4;
            }
            OPCODE_JAL => {
                let inst_j = instructions::_J(bits);
                self.set_register(inst_j.rd(), self.program_counter + 4);
                self.program_counter = self.program_counter.wrapping_add(inst_j.imm_dec() as u32);
            }
            OPCODE_JALR => {
                let inst_i = instructions::_I(bits);
                let oldpc = self.program_counter + 4;
                let pc = self
                    .get_register(inst_i.rs1())
                    .wrapping_add(inst_i.imm_dec() as u32);
                self.program_counter = pc & 0b1111_1111_1111_1111_1111_1111_1111_1110;
                self.set_register(inst_i.rd(), oldpc);
            }
            OPCODE_BRANCH => {
                self.execute_opcode_branch(bits);
            }
            OPCODE_LOAD => {
                self.execute_opcode_load(bits);
            }
            OPCODE_STORE => {
                self.execute_opcode_store(bits);
            }
            OPCODE_ALU_AND_SHIFT_IMM => {
                self.execute_opcode_alu_and_shift_imm(bits);
            }
            OPCODE_ALU_REGISTER => {
                self.execute_opcode_alu_register(bits);
            }
            OPCODE_E_AND_SYSTEM => {
                self.execute_opcode_e_and_system(bits);
            }
            _ => panic!("Unimplemented opcode {}", opcode),
        }
    }
    pub fn run(&mut self) {
        loop {
            self.set_register(0, 0);
            let bits = self.read_u32_memory(self.program_counter as usize);
            if bits == 0xDEADC0DE {
                break;
            }
            self.execute_inst(bits);
        }
    }
}

const REGISTERS_DUMP: &[&str; 32] = &[
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0/fp", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6",
];
