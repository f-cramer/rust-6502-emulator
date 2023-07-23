#![feature(fn_traits)]

use std::{env, fs};
use std::process::exit;

use crate::cpu::CPU;

mod cpu;
mod instructions;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("no file to interpret given");
        exit(1);
    }

    let path = &args[1];
    println!("reading from {}", path);
    let assembly = fs::read(path).unwrap();

    let mut cpu = CPU::new(assembly);
    cpu.pc = 0x400;

    loop {
        let result = cpu.execute();
        if result.is_err() {
            println!("next operation {:#04X}", cpu.fetch().unwrap());
        }
        result.expect("execute");
    }
}
