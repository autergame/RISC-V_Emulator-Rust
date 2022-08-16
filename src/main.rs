#[macro_use]
extern crate bitfield;

extern crate clap;

use std::fs::File;
use std::io::prelude::*;

mod cpu;
mod imm_enc_dec;
mod inst_defs;
mod instructions;
mod types;

mod assembler;

fn main() {
    let matches = clap::Command::new("RISC-V_Emulator-Rust")
        .version("0.1.0")
        .author("https://github.com/autergame/")
        .about("Simplest RISC-V Emulador in Rust")
        .arg_required_else_help(true)
        .subcommand(clap::SubCommand::with_name("help").hide(true))
        .subcommand(
            clap::Command::new("compile")
                .about("Assembles the given file")
                .arg(
                    clap::Arg::new("INPUT")
                        .help("Sets the input file to use")
                        .required(true)
                        .index(1),
                )
                .arg(
                    clap::Arg::new("OUTPUT")
                        .help("Sets the output file to use")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            clap::Command::new("run").about("Runs the given file").arg(
                clap::Arg::new("INPUT")
                    .help("Sets the input file to use")
                    .required(true)
                    .index(1),
            ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("compile") {
        let input = matches.value_of("INPUT").unwrap();
        let output = matches.value_of("OUTPUT").unwrap();
        let contents = read_to_string(input);
        let compiled_insts = assembler::assemble(&contents);
        write_u32(output, &compiled_insts);
    } else if let Some(matches) = matches.subcommand_matches("run") {
        let input = matches.value_of("INPUT").unwrap();
        let contents = read_to_u8(input);
        let mut cpu = cpu::RiscvCpu::new();
        cpu.load_from_u8(&contents);
        cpu.run();
    }
}

fn read_to_u8(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("Could not open file");
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Could not read file");
    contents
}

fn read_to_string(path: &str) -> String {
    let mut file = File::open(path).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");
    contents
}

fn write_u32(path: &str, v: &[u32]) {
    let mut file = File::create(path).expect("Could not create file");
    let v_u8 = v.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>();
    file.write_all(&v_u8).expect("Could not write file");
}
