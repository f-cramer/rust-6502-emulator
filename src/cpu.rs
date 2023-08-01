use std::fmt::{Debug, Formatter};

use crate::{instructions, utils};
use crate::instructions::run_instruction;

pub struct CPU {
    pub memory: Memory,

    pub a: i8,
    pub x: i8,
    pub y: i8,
    pub pc: u16,
    pub sp: u16,

    pub n: bool,
    pub v: bool,
    pub b: bool,
    pub d: bool,
    pub i: bool,
    pub z: bool,
    pub c: bool,

    pub instruction_count: u32,
}

impl CPU {
    pub fn new(data: Vec<u8>) -> CPU {
        CPU {
            memory: Memory { data },
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            n: false,
            v: false,
            b: false,
            d: false,
            i: false,
            z: false,
            c: false,
            instruction_count: 0,
        }
    }

    pub fn execute(&mut self) -> Result<(), String> {
        let operation = self.fetch()?;
        let instruction = instructions::parse_opcode(operation)?;
        if self.instruction_count % 1000000 == 0 {
            println!("{}: running instruction {:?} from {:#06X}", self.instruction_count, instruction, self.pc - 1);
        }
        run_instruction(&instruction, self)?;
        self.instruction_count = self.instruction_count + 1;
        Ok(())
    }

    pub fn fetch(&mut self) -> Result<u8, String> {
        let memory = self.memory.data[self.pc as usize];
        self.pc = self.pc + 1;
        Ok(memory)
    }

    pub fn get_sr(&self) -> u8 {
        (if self.n { 1 } else { 0 } << 7) +
            (if self.v { 1 } else { 0 } << 6) +
            (1 << 5) +
            (if self.b { 1 } else { 0 } << 4) +
            (if self.d { 1 } else { 0 } << 3) +
            (if self.i { 1 } else { 0 } << 2) +
            (if self.z { 1 } else { 0 } << 1) +
            (if self.c { 1 } else { 0 } << 0)
    }

    fn format_sr(&self) -> String {
        format!(
            "{:#010b} (n: {}, v: {}, b: {}, d: {}, i: {}, z: {}, c: {})",
            self.get_sr(),
            bool_to_u8(self.n),
            bool_to_u8(self.v),
            bool_to_u8(self.b),
            bool_to_u8(self.d),
            bool_to_u8(self.i),
            bool_to_u8(self.z),
            bool_to_u8(self.c),
        )
    }
}

impl Debug for CPU {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "CPU {{ a: {}, x: {}, y: {}, pc: {}, sp: {}, sr: {} }}",
            format_i8(self.a),
            format_i8(self.x),
            format_i8(self.y),
            format_u16(self.pc),
            format_u16(self.sp),
            self.format_sr(),
        )
    }
}

fn bool_to_u8(value: bool) -> u8 {
    if value { 1 } else { 0 }
}

fn format_i8(value: i8) -> String {
    format!("{}[{:#}, {:#04X}, {:#010b}]", value, value as u8, value, value)
}

fn format_u16(value: u16) -> String {
    format!("{} [{:#06X}]", value, value)
}

#[derive(Debug)]
pub struct Memory {
    data: Vec<u8>,
}

impl Memory {
    pub fn get(&self, lsb: u8, msb: u8, offset: u8) -> u8 {
        let address = utils::combine(lsb, msb, offset);
        self.get16(address)
    }

    pub fn get16(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn set(&mut self, lsb: u8, msb: u8, offset: u8, value: u8) {
        let address = utils::combine(lsb, msb, offset);
        self.set16(address, value);
    }

    pub fn set16(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }
}
