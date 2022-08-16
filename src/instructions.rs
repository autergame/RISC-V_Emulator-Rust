use imm_enc_dec::{
    inst_b_imm_dec, inst_b_imm_enc, inst_i_imm_dec, inst_i_imm_enc, inst_j_imm_dec, inst_j_imm_enc,
    inst_s_imm_dec, inst_s_imm_enc, inst_shift_imm_dec, inst_shift_imm_enc, inst_u_imm_dec,
    inst_u_imm_enc,
};

bitfield! {
    #[derive(Copy, Clone)]
    pub struct _Op(u32);
    impl Debug;
    u32;
    pub opcode, set_opcode :  6,  0;
    pub rest,   set_rest   : 31,  7;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct _R(u32);
    impl Debug;
    u32;
    pub opcode, set_opcode :  6,  0;
    pub rd,     set_rd     : 11,  7;
    pub funct3, set_funct3 : 14, 12;
    pub rs1,    set_rs1    : 19, 15;
    pub rs2,    set_rs2    : 24, 20;
    pub funct7, set_funct7 : 31, 25;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct _I(u32);
    impl Debug;
    u32;
    pub opcode,  set_opcode  :  6,  0;
    pub rd,      set_rd      : 11,  7;
    pub funct3,  set_funct3  : 14, 12;
    pub rs1,     set_rs1     : 19, 15;
    pub imm11_0, set_imm11_0 : 31, 20;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct _Shift(u32);
    impl Debug;
    u32;
    pub opcode,    set_opcode    :  6,  0;
    pub rd,        set_rd        : 11,  7;
    pub funct3,    set_funct3    : 14, 12;
    pub rs1,       set_rs1       : 19, 15;
    pub shamt_4_0, set_shamt_4_0 : 24, 20;
    pub funct7,    set_funct7    : 31, 25;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct _S(u32);
    impl Debug;
    u32;
    pub opcode,  set_opcode  :  6,  0;
    pub imm4_0,  set_imm4_0  : 11,  7;
    pub funct3,  set_funct3  : 14, 12;
    pub rs1,     set_rs1     : 19, 15;
    pub rs2,     set_rs2     : 24, 20;
    pub imm11_5, set_imm11_5 : 31, 25;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct _B(u32);
    impl Debug;
    u32;
    pub opcode,  set_opcode  :  6,  0;
    pub imm11,   set_imm11   :  7,  7;
    pub imm4_1,  set_imm4_1  : 11,  8;
    pub funct3,  set_funct3  : 14, 12;
    pub rs1,     set_rs1     : 19, 15;
    pub rs2,     set_rs2     : 24, 20;
    pub imm10_5, set_imm10_5 : 30, 25;
    pub imm12,   set_imm12   : 31, 31;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct _U(u32);
    impl Debug;
    u32;
    pub opcode,   set_opcode   :  6,  0;
    pub rd,       set_rd       : 11,  7;
    pub imm31_12, set_imm31_12 : 31, 12;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct _J(u32);
    impl Debug;
    u32;
    pub opcode,   set_opcode   :  6,  0;
    pub rd,       set_rd       : 11,  7;
    pub imm19_12, set_imm19_12 : 19, 12;
    pub imm11,    set_imm11    : 20, 20;
    pub imm10_1,  set_imm10_1  : 30, 21;
    pub imm20,    set_imm20    : 31, 31;
}

impl _I {
    pub fn imm_enc(&mut self, imm: i32) {
        inst_i_imm_enc(self, imm);
    }
    pub fn imm_dec(&self) -> i32 {
        inst_i_imm_dec(self)
    }
}

impl _Shift {
    pub fn imm_enc(&mut self, imm: u32) {
        inst_shift_imm_enc(self, imm);
    }
    pub fn imm_dec(&self) -> u32 {
        inst_shift_imm_dec(self)
    }
}

impl _S {
    pub fn imm_enc(&mut self, imm: i32) {
        inst_s_imm_enc(self, imm);
    }
    pub fn imm_dec(&self) -> i32 {
        inst_s_imm_dec(self)
    }
}

impl _B {
    pub fn imm_enc(&mut self, imm: i32) {
        inst_b_imm_enc(self, imm);
    }
    pub fn imm_dec(&self) -> i32 {
        inst_b_imm_dec(self)
    }
}

impl _U {
    pub fn imm_enc(&mut self, imm: u32) {
        inst_u_imm_enc(self, imm);
    }
    pub fn imm_dec(&self) -> u32 {
        inst_u_imm_dec(self)
    }
}

impl _J {
    pub fn imm_enc(&mut self, imm: i32) {
        inst_j_imm_enc(self, imm);
    }
    pub fn imm_dec(&self) -> i32 {
        inst_j_imm_dec(self)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    R(_R),
    I(_I),
    Shift(_Shift),
    S(_S),
    B(_B),
    U(_U),
    J(_J),
}

impl Instruction {
    pub fn get_bits(&self) -> u32 {
        match self {
            Instruction::R(r) => r.0,
            Instruction::I(i) => i.0,
            Instruction::Shift(s) => s.0,
            Instruction::S(s) => s.0,
            Instruction::B(b) => b.0,
            Instruction::U(u) => u.0,
            Instruction::J(j) => j.0,
        }
    }
    pub fn new_r(
        opcode: u32,
        funct3: u32,
        funct7: u32,
        rd: u32,
        rs1: u32,
        rs2: u32,
    ) -> Instruction {
        let mut r = _R(0);
        r.set_opcode(opcode);
        r.set_rd(rd);
        r.set_funct3(funct3);
        r.set_rs1(rs1);
        r.set_rs2(rs2);
        r.set_funct7(funct7);
        Instruction::R(r)
    }
    pub fn new_i(opcode: u32, funct3: u32, rd: u32, rs1: u32, imm: i32) -> Instruction {
        let mut i = _I(0);
        i.set_opcode(opcode);
        i.set_rd(rd);
        i.set_funct3(funct3);
        i.set_rs1(rs1);
        i.imm_enc(imm);
        Instruction::I(i)
    }
    pub fn new_shift(
        opcode: u32,
        funct3: u32,
        funct7: u32,
        rd: u32,
        rs1: u32,
        shamt: u32,
    ) -> Instruction {
        let mut s = _Shift(0);
        s.set_opcode(opcode);
        s.set_rd(rd);
        s.set_funct3(funct3);
        s.set_rs1(rs1);
        s.set_funct7(funct7);
        s.imm_enc(shamt);
        Instruction::Shift(s)
    }
    pub fn new_s(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: i32) -> Instruction {
        let mut s = _S(0);
        s.set_opcode(opcode);
        s.set_funct3(funct3);
        s.set_rs1(rs1);
        s.set_rs2(rs2);
        s.imm_enc(imm);
        Instruction::S(s)
    }
    pub fn new_b(opcode: u32, funct3: u32, rs1: u32, rs2: u32, imm: i32) -> Instruction {
        let mut b = _B(0);
        b.set_opcode(opcode);
        b.set_funct3(funct3);
        b.set_rs1(rs1);
        b.set_rs2(rs2);
        b.imm_enc(imm);
        Instruction::B(b)
    }
    pub fn new_u(opcode: u32, rd: u32, imm: u32) -> Instruction {
        let mut u = _U(0);
        u.set_opcode(opcode);
        u.set_rd(rd);
        u.imm_enc(imm << 12); // lower to upper
        Instruction::U(u)
    }
    pub fn new_j(opcode: u32, rd: u32, imm: i32) -> Instruction {
        let mut j = _J(0);
        j.set_opcode(opcode);
        j.set_rd(rd);
        j.imm_enc(imm);
        Instruction::J(j)
    }
}
