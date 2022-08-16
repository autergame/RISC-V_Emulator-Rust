use instructions::Instruction;
use types::*;

pub fn inst_lui(rd: u32, imm: u32) -> Instruction {
    Instruction::new_u(OPCODE_LUI, rd, imm)
}
pub fn inst_auipc(rd: u32, imm: u32) -> Instruction {
    Instruction::new_u(OPCODE_AUIPC, rd, imm)
}

pub fn inst_jal(rd: u32, imm: i32) -> Instruction {
    Instruction::new_j(OPCODE_JAL, rd, imm)
}
pub fn inst_jalr(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_JALR, FUNCT3_000, rd, rs1, imm)
}

pub fn inst_beq(rs1: u32, rs2: u32, imm: i32) -> Instruction {
    Instruction::new_b(OPCODE_BRANCH, FUNCT3_000, rs1, rs2, imm)
}
pub fn inst_bne(rs1: u32, rs2: u32, imm: i32) -> Instruction {
    Instruction::new_b(OPCODE_BRANCH, FUNCT3_001, rs1, rs2, imm)
}
pub fn inst_blt(rs1: u32, rs2: u32, imm: i32) -> Instruction {
    Instruction::new_b(OPCODE_BRANCH, FUNCT3_100, rs1, rs2, imm)
}
pub fn inst_bge(rs1: u32, rs2: u32, imm: i32) -> Instruction {
    Instruction::new_b(OPCODE_BRANCH, FUNCT3_101, rs1, rs2, imm)
}
pub fn inst_bltu(rs1: u32, rs2: u32, imm: i32) -> Instruction {
    Instruction::new_b(OPCODE_BRANCH, FUNCT3_110, rs1, rs2, imm)
}
pub fn inst_bgeu(rs1: u32, rs2: u32, imm: i32) -> Instruction {
    Instruction::new_b(OPCODE_BRANCH, FUNCT3_111, rs1, rs2, imm)
}

pub fn inst_lb(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_LOAD, FUNCT3_000, rd, rs1, imm)
}
pub fn inst_lh(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_LOAD, FUNCT3_001, rd, rs1, imm)
}
pub fn inst_lw(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_LOAD, FUNCT3_010, rd, rs1, imm)
}
pub fn inst_lbu(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_LOAD, FUNCT3_100, rd, rs1, imm)
}
pub fn inst_lhu(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_LOAD, FUNCT3_101, rd, rs1, imm)
}

pub fn inst_sb(rs1: u32, rs2: u32, imm: i32) -> Instruction {
    Instruction::new_s(OPCODE_STORE, FUNCT3_000, rs1, rs2, imm)
}
pub fn inst_sh(rs1: u32, rs2: u32, imm: i32) -> Instruction {
    Instruction::new_s(OPCODE_STORE, FUNCT3_001, rs1, rs2, imm)
}
pub fn inst_sw(rs1: u32, rs2: u32, imm: i32) -> Instruction {
    Instruction::new_s(OPCODE_STORE, FUNCT3_010, rs1, rs2, imm)
}

pub fn inst_addi(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_ALU_AND_SHIFT_IMM, FUNCT3_000, rd, rs1, imm)
}
pub fn inst_slti(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_ALU_AND_SHIFT_IMM, FUNCT3_010, rd, rs1, imm)
}
pub fn inst_sltiu(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_ALU_AND_SHIFT_IMM, FUNCT3_011, rd, rs1, imm)
}
pub fn inst_xori(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_ALU_AND_SHIFT_IMM, FUNCT3_100, rd, rs1, imm)
}
pub fn inst_ori(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_ALU_AND_SHIFT_IMM, FUNCT3_110, rd, rs1, imm)
}
pub fn inst_andi(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_ALU_AND_SHIFT_IMM, FUNCT3_111, rd, rs1, imm)
}
pub fn inst_slli(rd: u32, rs1: u32, shamt: u32) -> Instruction {
    Instruction::new_shift(
        OPCODE_ALU_AND_SHIFT_IMM,
        FUNCT3_001,
        FUNCT7_0000000,
        rd,
        rs1,
        shamt,
    )
}
pub fn inst_srli(rd: u32, rs1: u32, shamt: u32) -> Instruction {
    Instruction::new_shift(
        OPCODE_ALU_AND_SHIFT_IMM,
        FUNCT3_101,
        FUNCT7_0000000,
        rd,
        rs1,
        shamt,
    )
}
pub fn inst_srai(rd: u32, rs1: u32, shamt: u32) -> Instruction {
    Instruction::new_shift(
        OPCODE_ALU_AND_SHIFT_IMM,
        FUNCT3_101,
        FUNCT7_0100000,
        rd,
        rs1,
        shamt,
    )
}

pub fn inst_add(rd: u32, rs1: u32, rs2: u32) -> Instruction {
    Instruction::new_r(
        OPCODE_ALU_REGISTER,
        FUNCT3_000,
        FUNCT7_0000000,
        rd,
        rs1,
        rs2,
    )
}
pub fn inst_sub(rd: u32, rs1: u32, rs2: u32) -> Instruction {
    Instruction::new_r(
        OPCODE_ALU_REGISTER,
        FUNCT3_000,
        FUNCT7_0100000,
        rd,
        rs1,
        rs2,
    )
}
pub fn inst_sll(rd: u32, rs1: u32, rs2: u32) -> Instruction {
    Instruction::new_r(
        OPCODE_ALU_REGISTER,
        FUNCT3_001,
        FUNCT7_0000000,
        rd,
        rs1,
        rs2,
    )
}
pub fn inst_slt(rd: u32, rs1: u32, rs2: u32) -> Instruction {
    Instruction::new_r(
        OPCODE_ALU_REGISTER,
        FUNCT3_010,
        FUNCT7_0000000,
        rd,
        rs1,
        rs2,
    )
}
pub fn inst_sltu(rd: u32, rs1: u32, rs2: u32) -> Instruction {
    Instruction::new_r(
        OPCODE_ALU_REGISTER,
        FUNCT3_011,
        FUNCT7_0000000,
        rd,
        rs1,
        rs2,
    )
}
pub fn inst_xor(rd: u32, rs1: u32, rs2: u32) -> Instruction {
    Instruction::new_r(
        OPCODE_ALU_REGISTER,
        FUNCT3_100,
        FUNCT7_0000000,
        rd,
        rs1,
        rs2,
    )
}
pub fn inst_srl(rd: u32, rs1: u32, rs2: u32) -> Instruction {
    Instruction::new_r(
        OPCODE_ALU_REGISTER,
        FUNCT3_101,
        FUNCT7_0000000,
        rd,
        rs1,
        rs2,
    )
}
pub fn inst_sra(rd: u32, rs1: u32, rs2: u32) -> Instruction {
    Instruction::new_r(
        OPCODE_ALU_REGISTER,
        FUNCT3_101,
        FUNCT7_0100000,
        rd,
        rs1,
        rs2,
    )
}
pub fn inst_or(rd: u32, rs1: u32, rs2: u32) -> Instruction {
    Instruction::new_r(
        OPCODE_ALU_REGISTER,
        FUNCT3_110,
        FUNCT7_0000000,
        rd,
        rs1,
        rs2,
    )
}
pub fn inst_and(rd: u32, rs1: u32, rs2: u32) -> Instruction {
    Instruction::new_r(
        OPCODE_ALU_REGISTER,
        FUNCT3_111,
        FUNCT7_0000000,
        rd,
        rs1,
        rs2,
    )
}

pub fn inst_ecall() -> Instruction {
    Instruction::new_i(OPCODE_E_AND_SYSTEM, 0, 0, 0, 0)
}
pub fn inst_ebreak() -> Instruction {
    Instruction::new_i(OPCODE_E_AND_SYSTEM, 0, 0, 0, 1)
}

pub fn inst_csrrw(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_E_AND_SYSTEM, FUNCT3_001, rd, rs1, imm)
}
pub fn inst_csrrs(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_E_AND_SYSTEM, FUNCT3_010, rd, rs1, imm)
}
pub fn inst_csrrc(rd: u32, rs1: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_E_AND_SYSTEM, FUNCT3_011, rd, rs1, imm)
}
pub fn inst_csrrwi(rd: u32, zimm: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_E_AND_SYSTEM, FUNCT3_101, rd, zimm, imm)
}
pub fn inst_csrrsi(rd: u32, zimm: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_E_AND_SYSTEM, FUNCT3_110, rd, zimm, imm)
}
pub fn inst_csrrci(rd: u32, zimm: u32, imm: i32) -> Instruction {
    Instruction::new_i(OPCODE_E_AND_SYSTEM, FUNCT3_111, rd, zimm, imm)
}
