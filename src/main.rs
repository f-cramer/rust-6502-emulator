use std::{env, fs};
use std::process::exit;

use crate::cpu::{CPU, ExecutionFinished};

mod cpu;
mod instructions;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("no file to interpret given");
        exit(1);
    }
    if args.len() <= 2 {
        eprintln!("no success instruction given");
        exit(2);
    }

    let path = &args[1];
    let success_instruction = u16::from_str_radix(&args[2], 16).expect("cannot parse success_instruction to u16");

    println!("reading from {}", path);
    println!("success at instruction {:#06X}", success_instruction);
    let assembly = fs::read(path).unwrap();

    let mut cpu = CPU::new(assembly);
    cpu.pc = 0x400;

    loop {
        let result = cpu.execute(success_instruction);
        if result.is_err() {
            println!("next operation {:#04X} at {:#06X}", cpu.fetch().unwrap(), cpu.pc - 1);
            println!("cpu {:?}", cpu);
        }

        if result.expect("execute") == ExecutionFinished::YES {
            break;
        }
    }
}
