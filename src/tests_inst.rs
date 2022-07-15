use cpu::*;
use inst_defs::*;
use types::*;

fn test_lui(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 291), // 0x123
		inst_lui(R_T1, 0x12345),
		inst_add(R_T2, R_T0, R_T1),
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x123);
	assert_eq!(cpu.get_register(R_T1), 0x12345000);
	assert_eq!(cpu.get_register(R_T2), 0x12345123);
}

fn test_auipc(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 291), // 0x123
		inst_auipc(R_T1, 0x12345),    // 0x12345004
		inst_add(R_T2, R_T0, R_T1),   // 0x12345127
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x123);
	assert_eq!(cpu.get_register(R_T1), 0x12345004);
	assert_eq!(cpu.get_register(R_T2), 0x12345127);
}

fn test_jal(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 582), // 0x246
		inst_addi(R_T1, R_T0, 291),   // 0x123
		inst_jal(R_T2, 8),            // 0x00c
		inst_add(R_T2, R_T0, R_T1),   // 0x369
		inst_add(R_T3, R_T2, R_T1),   // 0x375
		inst_addi(R_T3, R_T3, 873),   // 0x6de
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x246);
	assert_eq!(cpu.get_register(R_T1), 0x369);
	assert_eq!(cpu.get_register(R_T2), 0x00c);
	assert_eq!(cpu.get_register(R_T3), 0x6de);
}

fn test_jalr(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 582), // 0x246
		inst_addi(R_T1, R_T0, 291),   // 0x123
		inst_addi(R_T2, R_ZERO, 12),  // 0x00c
		inst_jalr(R_T3, R_T2, 8),     // 0x010
		inst_add(R_T3, R_T0, R_T1),   // 0x369
		inst_add(R_T4, R_T3, R_T1),   // 0x379
		inst_addi(R_T4, R_T4, 873),   // 0x6e2
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x246);
	assert_eq!(cpu.get_register(R_T1), 0x369);
	assert_eq!(cpu.get_register(R_T2), 0x00c);
	assert_eq!(cpu.get_register(R_T3), 0x010);
	assert_eq!(cpu.get_register(R_T4), 0x6e2);
}

// region OPCODE_BRANCH

fn test_beq(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 291), // 0x123
		inst_addi(R_T1, R_T1, 291),   // 0x123
		inst_beq(R_T0, R_T1, -4),     // 1 == 2
		inst_add(R_T2, R_T0, R_T1),   // 0x369
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x123);
	assert_eq!(cpu.get_register(R_T1), 0x246);
	assert_eq!(cpu.get_register(R_T2), 0x369);
}

fn test_bne(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 582), // 0x246
		inst_addi(R_T1, R_T1, 291),   // 0x123
		inst_bne(R_T0, R_T1, -4),     // 1 != 2
		inst_add(R_T2, R_T0, R_T1),   // 0x48c
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x246);
	assert_eq!(cpu.get_register(R_T1), 0x246);
	assert_eq!(cpu.get_register(R_T2), 0x48c);
}

fn test_blt(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 582), // 0x246
		inst_addi(R_T1, R_T1, 291),   // 0x123
		inst_blt(R_T1, R_T0, -4),     // 2 < 1
		inst_add(R_T2, R_T0, R_T1),   // 0x48c
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x246);
	assert_eq!(cpu.get_register(R_T1), 0x246);
	assert_eq!(cpu.get_register(R_T2), 0x48c);
}

fn test_bge(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 291),   // 0x123
		inst_addi(R_T1, R_ZERO, -1164), // 0xfffffb74
		inst_addi(R_T1, R_T1, 582),     // 0x246
		inst_bge(R_T0, R_T1, -4),       // 1 >= 2
		inst_add(R_T2, R_T0, R_T1),     // 0x369
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x123);
	assert_eq!(cpu.get_register(R_T1), 0x246);
	assert_eq!(cpu.get_register(R_T2), 0x369);
}

fn test_bltu(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 1606), // 0x646
		inst_addi(R_T1, R_T1, 1315),   // 0x523
		inst_bltu(R_T1, R_T0, -4),     // 2 < 1
		inst_add(R_T2, R_T0, R_T1),    // 0x108c
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x646);
	assert_eq!(cpu.get_register(R_T1), 0xa46);
	assert_eq!(cpu.get_register(R_T2), 0x108c);
}

fn test_bgeu(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 1606), // 0x646
		inst_addi(R_T1, R_T1, 1315),   // 0x523
		inst_bgeu(R_T0, R_T1, -4),     // 1 >= 2
		inst_add(R_T2, R_T0, R_T1),    // 0x108c
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x646);
	assert_eq!(cpu.get_register(R_T1), 0xa46);
	assert_eq!(cpu.get_register(R_T2), 0x108c);
}

// region OPCODE_BRANCH

// region OPCODE_LOAD OPCODE_STORE

fn test_lbsb(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 12),   // 0x00c
		inst_addi(R_T1, R_ZERO, -127), // 0xffffff81
		inst_sb(R_T0, R_T1, 8),        // 0x014
		inst_lb(R_T2, R_T0, 8),        // 0x008
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x00c);
	assert_eq!(cpu.get_register(R_T1), 0xffffff81);
	assert_eq!(cpu.get_register(R_T2), 0xffffff81);
}

fn test_lhsh(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 12), // 0x00c
		//inst_"li"(R_T1, -32767),   // 0xffff8001
		inst_lui(R_T1, 0x00008),  // 0x8000
		inst_addi(R_T1, R_T1, 1), // 0x8001
		inst_sh(R_T0, R_T1, 12),  // 0x018
		inst_lh(R_T2, R_T0, 12),  // 0x00c
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x00c);
	assert_eq!(cpu.get_register(R_T1), 0x8001);
	assert_eq!(cpu.get_register(R_T2), 0xffff8001);
}

fn test_lwsw(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 12), // 0x00c
		//inst_"li"(R_T1, -2147483647), // 0x80000001
		inst_lui(R_T1, 0x80000),  // 0x80000000
		inst_addi(R_T1, R_T1, 1), // 0x80000001
		inst_sw(R_T0, R_T1, 12),  // 0x018
		inst_lw(R_T2, R_T0, 12),  // 0x00c
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x00c);
	assert_eq!(cpu.get_register(R_T1), 0x80000001);
	assert_eq!(cpu.get_register(R_T2), 0x80000001);
}

fn test_lbu(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 12),  // 0x00c
		inst_addi(R_T1, R_ZERO, 255), // 0xff
		inst_sb(R_T0, R_T1, 8),       // 0x014
		inst_lbu(R_T2, R_T0, 8),      // 0x008
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x00c);
	assert_eq!(cpu.get_register(R_T1), 0xff);
	assert_eq!(cpu.get_register(R_T2), 0xff);
}

fn test_lhu(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 12), // 0x00c
		//inst_"li"(R_T1, 65535),    // 0xffff
		inst_lui(R_T1, 0x00010),   // 0x10000
		inst_addi(R_T1, R_T1, -1), // 0xfff
		inst_sh(R_T0, R_T1, 12),   // 0x018
		inst_lhu(R_T2, R_T0, 12),  // 0x00c
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x00c);
	assert_eq!(cpu.get_register(R_T1), 0xffff);
	assert_eq!(cpu.get_register(R_T2), 0xffff);
}

// region OPCODE_LOAD OPCODE_STORE

// region OPCODE_ALU_AND_SHIFT_IMM

fn test_addi(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 291), // 0x123
		inst_addi(R_T1, R_T0, 582),   // 0x246
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x123);
	assert_eq!(cpu.get_register(R_T1), 0x369);
}

fn test_slti(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, -127), // 0xffffff81
		inst_slti(R_T1, R_T0, 291),    // 0x123
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0xffffff81);
	assert_eq!(cpu.get_register(R_T1), 0x1);
}

fn test_sltiu(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, -127), // 0xffffff81
		inst_sltiu(R_T1, R_T0, 3456),  // 0xd80
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0xffffff81);
	assert_eq!(cpu.get_register(R_T1), 0x0);
}

fn test_xori(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 291), // 0x123
		inst_xori(R_T1, R_T0, 582),   // 0x246
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x123);
	assert_eq!(cpu.get_register(R_T1), 0x365);
}

fn test_ori(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 291), // 0x123
		inst_ori(R_T1, R_T0, 582),    // 0x246
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x123);
	assert_eq!(cpu.get_register(R_T1), 0x367);
}

fn test_andi(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 291), // 0x123
		inst_andi(R_T1, R_T0, 582),   // 0x246
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x123);
	assert_eq!(cpu.get_register(R_T1), 0x002);
}

fn test_slli(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 582), // 0x246
		inst_slli(R_T1, R_T0, 16),    // 0x2460000
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x246);
	assert_eq!(cpu.get_register(R_T1), 0x2460000);
}

fn test_srli(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_lui(R_T0, 0x2460),    // 0x2460000
		inst_srli(R_T1, R_T0, 16), // 0x246
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x2460000);
	assert_eq!(cpu.get_register(R_T1), 0x246);
}

fn test_srai(cpu: &mut RiscvCpu) {
	let inst_list = [
		//inst_"li"(R_T0, -2147483647), // 0x80000001
		inst_lui(R_T0, 0x80000),   // 0x80000000
		inst_addi(R_T0, R_T0, 1),  // 0x80000001
		inst_srai(R_T1, R_T0, 16), // 0xffff8000
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x80000001);
	assert_eq!(cpu.get_register(R_T1), 0xffff8000);
}

// region OPCODE_ALU_AND_SHIFT_IMM

// region OPCODE_ALU_REGISTER

fn test_add(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 291), // 0x123
		inst_addi(R_T1, R_ZERO, 582), // 0x246
		inst_add(R_T2, R_T0, R_T1),   // 0x369
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x123);
	assert_eq!(cpu.get_register(R_T1), 0x246);
	assert_eq!(cpu.get_register(R_T2), 0x369);
}

fn test_sub(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 582), // 0x246
		inst_addi(R_T1, R_ZERO, 291), // 0x123
		inst_sub(R_T2, R_T0, R_T1),   // 0x123
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x246);
	assert_eq!(cpu.get_register(R_T1), 0x123);
	assert_eq!(cpu.get_register(R_T2), 0x123);
}

fn test_sll(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 582), // 0x246
		inst_addi(R_T1, R_ZERO, 16),  // 0x010
		inst_sll(R_T2, R_T0, R_T1),   // 0x2460000
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x246);
	assert_eq!(cpu.get_register(R_T1), 0x010);
	assert_eq!(cpu.get_register(R_T2), 0x2460000);
}

fn test_slt(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, -127), // 0xffffff81
		inst_addi(R_T1, R_ZERO, 291),  // 0x123
		inst_slt(R_T2, R_T0, R_T1),
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0xffffff81);
	assert_eq!(cpu.get_register(R_T1), 0x123);
	assert_eq!(cpu.get_register(R_T2), 0x1);
}

fn test_sltu(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 2456), // 0x998
		inst_addi(R_T1, R_ZERO, 3456), // 0xd80
		inst_sltu(R_T2, R_T0, R_T1),
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0xfffff998);
	assert_eq!(cpu.get_register(R_T1), 0xfffffd80);
	assert_eq!(cpu.get_register(R_T2), 0x1);
}

fn test_xor(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 291), // 0x123
		inst_addi(R_T1, R_ZERO, 582), // 0x246
		inst_xor(R_T2, R_T0, R_T1),   // 0x365
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x123);
	assert_eq!(cpu.get_register(R_T1), 0x246);
	assert_eq!(cpu.get_register(R_T2), 0x365);
}

fn test_srl(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_lui(R_T0, 0x2460),      // 0x2460000
		inst_addi(R_T1, R_ZERO, 16), // 0x010
		inst_srl(R_T2, R_T0, R_T1),  // 0x246
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x2460000);
	assert_eq!(cpu.get_register(R_T1), 0x010);
	assert_eq!(cpu.get_register(R_T2), 0x246);
}

fn test_sra(cpu: &mut RiscvCpu) {
	let inst_list = [
		//inst_"li"(R_T0, -2147483647), // 0x80000001
		inst_lui(R_T0, 0x80000),     // 0x80000000
		inst_addi(R_T0, R_T0, 1),    // 0x80000001
		inst_addi(R_T1, R_ZERO, 16), // 0x010
		inst_sra(R_T2, R_T0, R_T1),  // 0xffff8000
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x80000001);
	assert_eq!(cpu.get_register(R_T1), 0x010);
	assert_eq!(cpu.get_register(R_T2), 0xffff8000);
}

fn test_or(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 291), // 0x123
		inst_addi(R_T1, R_ZERO, 582), // 0x246
		inst_or(R_T2, R_T0, R_T1),    // 0x367
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x123);
	assert_eq!(cpu.get_register(R_T1), 0x246);
	assert_eq!(cpu.get_register(R_T2), 0x367);
}

fn test_and(cpu: &mut RiscvCpu) {
	let inst_list = [
		inst_addi(R_T0, R_ZERO, 291), // 0x123
		inst_addi(R_T1, R_ZERO, 582), // 0x246
		inst_and(R_T2, R_T0, R_T1),   // 0x002
	];

	cpu.load_and_run(&inst_list);

	assert_eq!(cpu.get_register(R_T0), 0x123);
	assert_eq!(cpu.get_register(R_T1), 0x246);
	assert_eq!(cpu.get_register(R_T2), 0x002);
}

// region OPCODE_ALU_REGISTER

pub fn tests_inst(cpu: &mut RiscvCpu) {
	test_lui(cpu);
	test_auipc(cpu);

	test_jal(cpu);
	test_jalr(cpu);

	test_beq(cpu);
	test_bne(cpu);
	test_blt(cpu);
	test_bge(cpu);
	test_bltu(cpu);
	test_bgeu(cpu);

	test_lbsb(cpu);
	test_lhsh(cpu);
	test_lwsw(cpu);
	test_lbu(cpu);
	test_lhu(cpu);

	test_addi(cpu);
	test_slti(cpu);
	test_sltiu(cpu);
	test_xori(cpu);
	test_ori(cpu);
	test_andi(cpu);
	test_slli(cpu);
	test_srli(cpu);
	test_srai(cpu);

	test_add(cpu);
	test_sub(cpu);
	test_sll(cpu);
	test_slt(cpu);
	test_sltu(cpu);
	test_xor(cpu);
	test_srl(cpu);
	test_sra(cpu);
	test_or(cpu);
	test_and(cpu);
}

impl Instruction {
	pub fn load_from_instructions(&mut self, inst_list: &[Instruction]) {
		self.reset();
		inst_list.iter().enumerate().for_each(|(i, inst)| {
			self.write_u32_memory(i * 4, inst.get_bits());
		});
		self.write_u32_memory(inst_list.len() * 4, 0xDEADC0DE);
	}
	pub fn load_and_run(&mut self, inst_list: &[Instruction]) {
		self.load_from_instructions(inst_list);
		self.run();
	}
}