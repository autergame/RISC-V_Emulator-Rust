#[macro_use]
extern crate bitfield;

extern crate clap;

use std::{fs::File, io::prelude::*, path::Path};

mod cpu;
mod imm_enc_dec;
mod inst_defs;
mod instructions;
mod types;

mod assembler;

fn main() {
    let matches = clap::Command::new("RISC-V_Emulator-Rust")
        .version("0.2.0")
        .author("https://github.com/autergame/")
        .about("Simplest RISC-V Emulador Using Rust")
        .arg_required_else_help(true)
        .subcommand_required(true)
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

    match matches.subcommand() {
        Some(("compile", args)) => {
            let input = args.get_one::<String>("INPUT").unwrap();
            let output = args.get_one::<String>("OUTPUT").unwrap();

            let contents = read_string(Path::new(input));
            let compiled_insts = assembler::assemble(&contents);
            write_u32(Path::new(output), &compiled_insts);
        }
        Some(("run", args)) => {
            let input = args.get_one::<String>("INPUT").unwrap();

            let contents = read_to_u8(Path::new(input));
            let mut cpu = cpu::RiscvCpu::new();
            cpu.load_from_u8(&contents);
            cpu.run();
        }
        _ => {}
    }
}

fn read_to_u8(path: &Path) -> Vec<u8> {
    let mut file = File::open(path).expect("Could not open file");
    let mut contents: Vec<u8> = Vec::new();
    println!("Reading file: {}", path.to_str().unwrap());
    file.read_to_end(&mut contents)
        .expect("Could not read file");
    println!("Finished reading file");
    contents
}

fn read_string(path: &Path) -> String {
    let mut file = File::open(path).expect("Could not open file");
    let mut contents = String::new();
    println!("Reading file: {}", path.to_str().unwrap());
    file.read_to_string(&mut contents)
        .expect("Could not read file");
    println!("Finished reading file");
    contents
}

fn write_u32(path: &Path, v: &[u32]) {
    let mut file = File::create(path).expect("Could not create file");
    let v_u8 = v.iter().flat_map(|x| x.to_le_bytes()).collect::<Vec<u8>>();
    println!("Writing to file: {}", path.to_str().unwrap());
    file.write_all(&v_u8).expect("Could not write file");
    println!("Finished writing to file");
}
