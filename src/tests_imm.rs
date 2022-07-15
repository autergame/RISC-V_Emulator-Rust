use imm_enc_dec::*;
use instructions::*;

fn test_imm_s(
	name: &str,
	inst: u32,
	start: i32,
	end: i32,
	x_imm_enc: fn(i32) -> u32,
	x_imm_dec: fn(u32) -> i32,
	inc: usize,
) {
	for x in (start..=end).step_by(inc) {
		let inst_x_enc: u32 = x_imm_enc(x);
		let inst_x: u32 = inst_x_enc | inst;
		let inst_x_dec: i32 = x_imm_dec(inst_x);
		assert_eq!(x, inst_x_dec, "{0}", name);
	}
}

fn test_imm_u(
	name: &str,
	inst: u32,
	start: u32,
	end: u32,
	x_imm_enc: fn(u32) -> u32,
	x_imm_dec: fn(u32) -> u32,
	inc: usize,
) {
	for x in (start..=end).step_by(inc) {
		let inst_x_enc: u32 = x_imm_enc(x);
		let inst_x: u32 = inst_x_enc | inst;
		let inst_x_dec: u32 = x_imm_dec(inst_x);
		assert_eq!(x, inst_x_dec, "{0}", name);
	}
}

pub fn tests_imm() {
	let inst_imm_i: u32 = 0b000000000000_11111_111_11111_1111111;
	let inst_imm_s: u32 = 0b0000000_11111_11111_111_00000_1111111;
	let inst_imm_b: u32 = 0b0000000_11111_11111_111_00000_1111111;
	let inst_imm_u: u32 = 0b00000000000000000000_11111_1111111;
	let inst_imm_j: u32 = 0b00000000000000000000_11111_1111111;

	test_imm_s("I", inst_imm_i, -2048, 2047, i_imm_enc, i_imm_dec, 1);
	test_imm_s("S", inst_imm_s, -2048, 2047, s_imm_enc, s_imm_dec, 1);
	test_imm_s("B", inst_imm_b, -4096, 4095, b_imm_enc, b_imm_dec, 2);
	test_imm_u(
		"U", inst_imm_u, 0x00000000, 0xffffe000, u_imm_enc, u_imm_dec, 0x1000,
	);
	test_imm_s("J", inst_imm_j, -1048576, 1048575, j_imm_enc, j_imm_dec, 2);

	let inst_imm_shift: u32 = 0b1111111_00000_11111_111_11111_1111111;
	test_imm_u(
		"Shift",
		inst_imm_shift,
		0,
		31,
		shift_imm_enc,
		shift_imm_dec,
		1,
	);
}

fn test_inst_imm_s(name: &str, inst: &Instruction, start: i32, end: i32, inc: usize) {
	for x in (start..=end).step_by(inc) {
		let mut inst_x_enc = *inst;
		inst_x_enc.imm_enc_s(x);
		let inst_x_dec: i32 = inst_x_enc.imm_dec_s();
		assert_eq!(x, inst_x_dec, "{0}", name);
	}
}

fn test_inst_imm_u(name: &str, inst: &Instruction, start: u32, end: u32, inc: usize) {
	for x in (start..=end).step_by(inc) {
		let mut inst_x_enc = *inst;
		inst_x_enc.imm_enc_u(x);
		let inst_x_dec: u32 = inst_x_enc.imm_dec_u();
		assert_eq!(x, inst_x_dec, "{0}", name);
	}
}

pub fn tests_inst_imm() {
	let inst_imm_i = Instruction::new_i(0b1111111, 0b111, 31, 31, 0b000000000000); // 000000000000_11111_111_11111_1111111
	let inst_imm_s = Instruction::new_s(0b1111111, 0b111, 31, 31, 0b000000000000); // 0000000_11111_11111_111_00000_1111111
	let inst_imm_b = Instruction::new_b(0b1111111, 0b111, 31, 31, 0b000000000000); // 0000000_11111_11111_111_00000_1111111
	let inst_imm_u = Instruction::new_u(0b1111111, 31, 0b00000000000000000000); // 00000000000000000000_11111_1111111
	let inst_imm_j = Instruction::new_j(0b1111111, 31, 0b00000000000000000000); // 00000000000000000000_11111_1111111

	test_inst_imm_s("I", &inst_imm_i, -2048, 2047, 1);
	test_inst_imm_s("S", &inst_imm_s, -2048, 2047, 1);
	test_inst_imm_s("B", &inst_imm_b, -4096, 4095, 2);
	test_inst_imm_u("U", &inst_imm_u, 0x00000000, 0xffffe000, 0x1000);
	test_inst_imm_s("J", &inst_imm_j, -1048576, 1048575, 2);

	let inst_imm_shift = Instruction::new_shift(0b1111111, 0b111, 0b1111111, 31, 31, 0b00000); // 1111111_00000_11111_111_11111_1111111
	test_inst_imm_u("Shift", &inst_imm_shift, 0, 31, 1);
}

impl Instruction {
	pub fn imm_enc_s(&mut self, imm: i32) {
		match self {
			Instruction::I(i) => inst_i_imm_enc(i, imm),
			Instruction::S(s) => inst_s_imm_enc(s, imm),
			Instruction::B(b) => inst_b_imm_enc(b, imm),
			Instruction::J(j) => inst_j_imm_enc(j, imm),
			_ => panic!("imm_enc_s: not an instruction with an immediate signed"),
		}
	}
	pub fn imm_enc_u(&mut self, imm: u32) {
		match self {
			Instruction::U(u) => inst_u_imm_enc(u, imm),
			Instruction::Shift(shift) => inst_shift_imm_enc(shift, imm),
			_ => panic!("imm_enc_u: not an instruction with an immediate unsigned"),
		}
	}
	pub fn imm_dec_s(&mut self) -> i32 {
		return match self {
			Instruction::I(i) => inst_i_imm_dec(i),
			Instruction::S(s) => inst_s_imm_dec(s),
			Instruction::B(b) => inst_b_imm_dec(b),
			Instruction::J(j) => inst_j_imm_dec(j),
			_ => panic!("imm_dec_s: not an instruction with an immediate signed"),
		};
	}
	pub fn imm_dec_u(&mut self) -> u32 {
		return match self {
			Instruction::U(u) => inst_u_imm_dec(u),
			Instruction::Shift(shift) => inst_shift_imm_dec(shift),
			_ => panic!("imm_dec_u: not an instruction with an immediate unsigned"),
		};
	}
}
