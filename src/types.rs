#![allow(dead_code)]

pub const OPCODE_LUI: u32 = 0b0110111;
pub const OPCODE_AUIPC: u32 = 0b0010111;

pub const OPCODE_JAL: u32 = 0b1101111;
pub const OPCODE_JALR: u32 = 0b1100111;

// beq bne blt bge bltu bgeu
pub const OPCODE_BRANCH: u32 = 0b1100011;

// lb lh lw lbu lhu
pub const OPCODE_LOAD: u32 = 0b0000011;

// sb sh sw
pub const OPCODE_STORE: u32 = 0b0100011;

// addi slti sltiu xori ori andi slli srli srai
pub const OPCODE_ALU_AND_SHIFT_IMM: u32 = 0b0010011;

// add sub sll slt sltu xor srl sra or and
pub const OPCODE_ALU_REGISTER: u32 = 0b0110011;

// ecall ebreak csrrw csrrs csrrc csrrwi csrrsi csrrci
pub const OPCODE_E_AND_SYSTEM: u32 = 0b1110011;

// jalr beq lb sb addi add sub ecall ebreak
pub const FUNCT3_000: u32 = 0b000;

// bne lh sh slli sll csrrw
pub const FUNCT3_001: u32 = 0b001;

// lw sw slti slt csrrs
pub const FUNCT3_010: u32 = 0b010;

// sltiu sltu csrrc
pub const FUNCT3_011: u32 = 0b011;

// blt lbu xori xor
pub const FUNCT3_100: u32 = 0b100;

// bge lhu srli srai srl sra csrrwi
pub const FUNCT3_101: u32 = 0b101;

// bltu ori or csrrsi
pub const FUNCT3_110: u32 = 0b110;

// bgeu andi and csrrci
pub const FUNCT3_111: u32 = 0b111;

// slli srli add sll slt sltu xor or and srl
pub const FUNCT7_0000000: u32 = 0b0000000;

// srai sub sra
pub const FUNCT7_0100000: u32 = 0b0100000;

// ecall
pub const IMM11_0_000000000000: u32 = 0b000000000000;

// ebreak
pub const IMM11_0_000000000001: u32 = 0b000000000001;

// hard-wired zero
pub const R_ZERO: u32 = 0b00000;

// return address
pub const R_RA: u32 = 0b00001;

// stack pointer
pub const R_SP: u32 = 0b00010;

// global pointer
pub const R_GP: u32 = 0b00011;

// thread pointer
pub const R_TP: u32 = 0b00100;

// temporaries
pub const R_T0: u32 = 0b00101;
pub const R_T1: u32 = 0b00110;
pub const R_T2: u32 = 0b00111;

// saved register / frame pointer
pub const R_S0: u32 = 0b01000;
pub const R_FP: u32 = 0b01000;

// saved register
pub const R_S1: u32 = 0b01001;

// function arguments / return values
pub const R_A0: u32 = 0b01010;
pub const R_A1: u32 = 0b01011;

// function arguments
pub const R_A2: u32 = 0b01100;
pub const R_A3: u32 = 0b01101;
pub const R_A4: u32 = 0b01110;
pub const R_A5: u32 = 0b01111;
pub const R_A6: u32 = 0b10000;
pub const R_A7: u32 = 0b10001;

// saved registers
pub const R_S2: u32 = 0b10010;
pub const R_S3: u32 = 0b10011;
pub const R_S4: u32 = 0b10100;
pub const R_S5: u32 = 0b10101;
pub const R_S6: u32 = 0b10110;
pub const R_S7: u32 = 0b10111;
pub const R_S8: u32 = 0b11000;
pub const R_S9: u32 = 0b11001;
pub const R_S10: u32 = 0b11010;
pub const R_S11: u32 = 0b11011;

// temporaries
pub const R_T3: u32 = 0b11100;
pub const R_T4: u32 = 0b11101;
pub const R_T5: u32 = 0b11110;
pub const R_T6: u32 = 0b11111;
