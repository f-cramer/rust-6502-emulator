use crate::cpu::CPU;
use crate::instructions::Instruction::*;
use crate::utils;

macro_rules! define_instructions {
    ( $( $name:ident $opcode:literal ),* $(,)?) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        pub enum Instruction {
            $(
                $name,
            )*
            NOP { byte_size: u8 },
        }

        pub fn parse_opcode(opcode: u8) -> Result<Instruction, String> {
            let instruction = match opcode {
                $(
                    $opcode => Instruction::$name,
                )*
                0xEA => Instruction::NOP { byte_size: 1 },
                0x02 | 0x22 | 0x42 | 0x44 | 0x54 | 0x62 | 0x82 | 0xC2 | 0xD4 | 0xE2 | 0xF4 => Instruction::NOP { byte_size: 2 },
                0x5C | 0xDC | 0xFC => Instruction::NOP { byte_size: 3 },
                o => {
                    let lower_nibble = o & 0x0F;
                    if lower_nibble == 0x03 || lower_nibble == 0x07 || lower_nibble == 0x0B || lower_nibble == 0x0F {
                        Instruction::NOP { byte_size: 1 }
                    } else {
                        return Err(format!("unknown opcode {:#04X}", opcode))
                    }
                }
            };
            Ok(instruction)
        }
    };
}

define_instructions!(
    ADC_ABS 0x6D,
    ADC_ABSX 0x7D,
    ADC_ABSY 0x79,
    ADC_IMM 0x69,
    ADC_IND 0x72,
    ADC_INDX 0x61,
    ADC_INDY 0x71,
    ADC_ZP 0x65,
    ADC_ZPX 0x75,
    AND_ABS 0x2D,
    AND_ABSX 0x3D,
    AND_ABSY 0x39,
    AND_IMM 0x29,
    AND_IND 0x32,
    AND_INDX 0x21,
    AND_INDY 0x31,
    AND_ZP 0x25,
    AND_ZPX 0x35,
    ASL_ABS 0x0E,
    ASL_ABSX 0x1E,
    ASL_ACC 0x0A,
    ASL_ZP 0x06,
    ASL_ZPX 0x16,
    BBR0 0x0F,
    BBR1 0x1F,
    BBR2 0x2F,
    BBR3 0x3F,
    BBR4 0x4F,
    BBR5 0x5F,
    BBR6 0x6F,
    BBR7 0x7F,
    BBS0 0x8F,
    BBS1 0x9F,
    BBS2 0xAF,
    BBS3 0xBF,
    BBS4 0xCF,
    BBS5 0xDF,
    BBS6 0xEF,
    BBS7 0xFF,
    BCC 0x90,
    BCS 0xB0,
    BEQ 0xF0,
    BIT_ABS 0x2C,
    BIT_ABSX 0x3C,
    BIT_IMM 0x89,
    BIT_ZP 0x24,
    BIT_ZPX 0x34,
    BMI 0x30,
    BNE 0xD0,
    BPL 0x10,
    BRA 0x80,
    BRK 0x00,
    BVC 0x50,
    BVS 0x70,
    CLC 0x18,
    CLD 0xD8,
    CLI 0x58,
    CLV 0xB8,
    CMP_ABS 0xCD,
    CMP_ABSX 0xDD,
    CMP_ABSY 0xD9,
    CMP_IMM 0xC9,
    CMP_IND 0xD2,
    CMP_INDX 0xC1,
    CMP_INDY 0xD1,
    CMP_ZP 0xC5,
    CMP_ZPX 0xD5,
    CPX_ABS 0xEC,
    CPX_IMM 0xE0,
    CPX_ZP 0xE4,
    CPY_ABS 0xCC,
    CPY_IMM 0xC0,
    CPY_ZP 0xC4,
    DEC_ABS 0xCE,
    DEC_ABSX 0xDE,
    DEC_ACC 0x3A,
    DEC_ZP 0xC6,
    DEC_ZPX 0xD6,
    DEX 0xCA,
    DEY 0x88,
    EOR_ABS 0x4D,
    EOR_ABSX 0x5D,
    EOR_ABSY 0x59,
    EOR_IMM 0x49,
    EOR_IND 0x52,
    EOR_INDX 0x41,
    EOR_INDY 0x51,
    EOR_ZP 0x45,
    EOR_ZPX 0x55,
    INC_ABS 0xEE,
    INC_ABSX 0xFE,
    INC_ACC 0x1A,
    INC_ZP 0xE6,
    INC_ZPX 0xF6,
    INX 0xE8,
    INY 0xC8,
    JMP_ABS 0x4C,
    JMP_ABSX 0x7C,
    JMP_IND 0x6C,
    JSR 0x20,
    LDA_ABS 0xAD,
    LDA_ABSX 0xBD,
    LDA_ABSY 0xB9,
    LDA_IMM 0xA9,
    LDA_IND 0xB2,
    LDA_INDX 0xA1,
    LDA_INDY 0xB1,
    LDA_ZP 0xA5,
    LDA_ZPX 0xB5,
    LDX_ABS 0xAE,
    LDX_ABSY 0xBE,
    LDX_IMM 0xA2,
    LDX_ZP 0xA6,
    LDX_ZPY 0xB6,
    LDY_ABS 0xAC,
    LDY_ABSX 0xBC,
    LDY_IMM 0xA0,
    LDY_ZP 0xA4,
    LDY_ZPX 0xB4,
    LSR_ABS 0x4E,
    LSR_ABSX 0x5E,
    LSR_ACC 0x4A,
    LSR_ZP 0x46,
    LSR_ZPX 0x56,
    ORA_ABS 0x0D,
    ORA_ABSX 0x1D,
    ORA_ABSY 0x19,
    ORA_IMM 0x09,
    ORA_IND 0x12,
    ORA_INDX 0x01,
    ORA_INDY 0x11,
    ORA_ZP 0x05,
    ORA_ZPX 0x15,
    PHA 0x48,
    PHP 0x08,
    PHX 0xDA,
    PHY 0x5A,
    PLA 0x68,
    PLP 0x28,
    PLX 0xFA,
    PLY 0x7A,
    RMB0 0x07,
    RMB1 0x17,
    RMB2 0x27,
    RMB3 0x37,
    RMB4 0x47,
    RMB5 0x57,
    RMB6 0x67,
    RMB7 0x77,
    ROL_ABS 0x2E,
    ROL_ABSX 0x3E,
    ROL_ACC 0x2A,
    ROL_ZP 0x26,
    ROL_ZPX 0x36,
    ROR_ABS 0x6E,
    ROR_ABSX 0x7E,
    ROR_ACC 0x6A,
    ROR_ZP 0x66,
    ROR_ZPX 0x76,
    RTI 0x40,
    RTS 0x60,
    SBC_ABS 0xED,
    SBC_ABSX 0xFD,
    SBC_ABSY 0xF9,
    SBC_IMM 0xE9,
    SBC_IND 0xF2,
    SBC_INDX 0xE1,
    SBC_INDY 0xF1,
    SBC_ZP 0xE5,
    SBC_ZPX 0xF5,
    SEC 0x38,
    SED 0xF8,
    SEI 0x78,
    SMB0 0x87,
    SMB1 0x97,
    SMB2 0xA7,
    SMB3 0xB7,
    SMB4 0xC7,
    SMB5 0xD7,
    SMB6 0xE7,
    SMB7 0xF7,
    STA_ABS 0x8D,
    STA_ABSX 0x9D,
    STA_ABSY 0x99,
    STA_IND 0x92,
    STA_INDX 0x81,
    STA_INDY 0x91,
    STA_ZP 0x85,
    STA_ZPX 0x95,
    STX_ABS 0x8E,
    STX_ZP 0x86,
    STX_ZPY 0x96,
    STY_ABS 0x8C,
    STY_ZP 0x84,
    STY_ZPX 0x94,
    STZ_ABS 0x9C,
    STZ_ABSX 0x9E,
    STZ_ZP 0x64,
    STZ_ZPX 0x74,
    TAX 0xAA,
    TAY 0xA8,
    TRB_ABS 0x1C,
    TRB_ZP 0x14,
    TSB_ABS 0x0C,
    TSB_ZP 0x04,
    TSX 0xBA,
    TXA 0x8A,
    TXS 0x9A,
    TYA 0x98,
);

pub fn run_instruction(instruction: &Instruction, cpu: &mut CPU) -> Result<(), String> {
    match instruction {
        ADC_ABS => {
            let value = cpu.load_absolute()?;
            cpu.add_with_carry(value);
        }
        ADC_ABSX => {
            let value = cpu.load_absolute_x()?;
            cpu.add_with_carry(value);
        }
        ADC_ABSY => {
            let value = cpu.load_absolute_y()?;
            cpu.add_with_carry(value);
        }
        ADC_IMM => {
            let value = cpu.load_immediate()?;
            cpu.add_with_carry(value);
        }
        ADC_IND => {
            let value = cpu.load_indirect()?;
            cpu.add_with_carry(value);
        }
        ADC_INDX => {
            let value = cpu.load_indirect_x()?;
            cpu.add_with_carry(value);
        }
        ADC_INDY => {
            let value = cpu.load_indirect_y()?;
            cpu.add_with_carry(value);
        }
        ADC_ZP => {
            let value = cpu.load_zeropage()?;
            cpu.add_with_carry(value);
        }
        ADC_ZPX => {
            let value = cpu.load_zeropage_x()?;
            cpu.add_with_carry(value);
        }
        AND_ABS => {
            let value = cpu.load_absolute()?;
            cpu.and(value);
        }
        AND_ABSX => {
            let value = cpu.load_absolute_x()?;
            cpu.and(value);
        }
        AND_ABSY => {
            let value = cpu.load_absolute_y()?;
            cpu.and(value);
        }
        AND_IMM => {
            let value = cpu.load_immediate()?;
            cpu.and(value);
        }
        AND_IND => {
            let value = cpu.load_indirect()?;
            cpu.and(value);
        }
        AND_INDX => {
            let value = cpu.load_indirect_x()?;
            cpu.and(value);
        }
        AND_INDY => {
            let value = cpu.load_indirect_y()?;
            cpu.and(value);
        }
        AND_ZP => {
            let value = cpu.load_zeropage()?;
            cpu.and(value);
        }
        AND_ZPX => {
            let value = cpu.load_zeropage_x()?;
            cpu.and(value);
        }
        ASL_ABS => cpu.load_store_absolute(|(c, value)| c.asl(value))?,
        ASL_ABSX => cpu.load_store_absolute_x(|(c, value)| c.asl(value))?,
        ASL_ACC => {
            let value = cpu.asl(cpu.a);
            cpu.a = value; // status has already been set in asl
        }
        ASL_ZP => cpu.load_store_zeropage(|(c, value)| c.asl(value))?,
        ASL_ZPX => cpu.load_store_zeropage_x(|(c, value)| c.asl(value))?,
        BBR0 => cpu.branch_if_bit_reset(0)?,
        BBR1 => cpu.branch_if_bit_reset(1)?,
        BBR2 => cpu.branch_if_bit_reset(2)?,
        BBR3 => cpu.branch_if_bit_reset(3)?,
        BBR4 => cpu.branch_if_bit_reset(4)?,
        BBR5 => cpu.branch_if_bit_reset(5)?,
        BBR6 => cpu.branch_if_bit_reset(6)?,
        BBR7 => cpu.branch_if_bit_reset(7)?,
        BBS0 => cpu.branch_if_bit_set(0)?,
        BBS1 => cpu.branch_if_bit_set(1)?,
        BBS2 => cpu.branch_if_bit_set(2)?,
        BBS3 => cpu.branch_if_bit_set(3)?,
        BBS4 => cpu.branch_if_bit_set(4)?,
        BBS5 => cpu.branch_if_bit_set(5)?,
        BBS6 => cpu.branch_if_bit_set(6)?,
        BBS7 => cpu.branch_if_bit_set(7)?,
        BCC => cpu.branch(!cpu.c)?,
        BCS => cpu.branch(cpu.c)?,
        BEQ => cpu.branch(cpu.z)?,
        BIT_ABS => {
            let value = cpu.load_absolute()?;
            cpu.test_bit(value, true);
        }
        BIT_ABSX => {
            let value = cpu.load_absolute_x()?;
            cpu.test_bit(value, true);
        }
        BIT_IMM => {
            let value = cpu.load_immediate()?;
            cpu.test_bit(value, false);
        }
        BIT_ZP => {
            let value = cpu.load_zeropage()?;
            cpu.test_bit(value, true);
        }
        BIT_ZPX => {
            let value = cpu.load_zeropage_x()?;
            cpu.test_bit(value, true);
        }
        BMI => cpu.branch(cpu.n)?,
        BNE => cpu.branch(!cpu.z)?,
        BPL => cpu.branch(!cpu.n)?,
        BRA => cpu.branch(true)?,
        BRK => {
            cpu.fetch()?; // ignore value

            let target_pc = cpu.pc;
            cpu.push((target_pc >> 8) as u8);
            cpu.b = true;
            cpu.push(target_pc as u8);

            let sr = cpu.get_sr();
            cpu.push(sr);
            cpu.i = true;
            cpu.d = false;

            let lsb = cpu.memory.get16(0xFFFE);
            let msb = cpu.memory.get16(0xFFFF);
            let new_pc = utils::combine(lsb, msb, 0);
            cpu.pc = new_pc;
        }
        BVC => cpu.branch(!cpu.v)?,
        BVS => cpu.branch(cpu.v)?,
        CLC => cpu.c = false,
        CLD => cpu.d = false,
        CLI => cpu.i = false,
        CLV => cpu.v = false,
        CMP_ABS => {
            let status = cpu.a.wrapping_sub(cpu.load_absolute()?);
            cpu.set_status_compare(status);
        }
        CMP_ABSX => {
            let status = cpu.a.wrapping_sub(cpu.load_absolute_x()?);
            cpu.set_status_compare(status);
        }
        CMP_ABSY => {
            let status = cpu.a.wrapping_sub(cpu.load_absolute_y()?);
            cpu.set_status_compare(status);
        }
        CMP_IMM => {
            let status = cpu.a.wrapping_sub(cpu.load_immediate()?);
            cpu.set_status_compare(status);
        }
        CMP_IND => {
            let status = cpu.a.wrapping_sub(cpu.load_indirect()?);
            cpu.set_status_compare(status);
        }
        CMP_INDX => {
            let status = cpu.a.wrapping_sub(cpu.load_indirect_x()?);
            cpu.set_status_compare(status);
        }
        CMP_INDY => {
            let status = cpu.a.wrapping_sub(cpu.load_indirect_y()?);
            cpu.set_status_compare(status);
        }
        CMP_ZP => {
            let status = cpu.a.wrapping_sub(cpu.load_zeropage()?);
            cpu.set_status_compare(status);
        }
        CMP_ZPX => {
            let status = cpu.a.wrapping_sub(cpu.load_zeropage_x()?);
            cpu.set_status_compare(status);
        }
        CPX_ABS => {
            let status = cpu.x.wrapping_sub(cpu.load_absolute()?);
            cpu.set_status_compare(status);
        }
        CPX_IMM => {
            let status = cpu.x.wrapping_sub(cpu.load_immediate()?);
            cpu.set_status_compare(status);
        }
        CPX_ZP => {
            let status = cpu.x.wrapping_sub(cpu.load_zeropage()?);
            cpu.set_status_compare(status);
        }
        CPY_ABS => {
            let status = cpu.y.wrapping_sub(cpu.load_absolute()?);
            cpu.set_status_compare(status);
        }
        CPY_IMM => {
            let status = cpu.y.wrapping_sub(cpu.load_immediate()?);
            cpu.set_status_compare(status);
        }
        CPY_ZP => {
            let status = cpu.y.wrapping_sub(cpu.load_zeropage()?);
            cpu.set_status_compare(status);
        }
        DEC_ABS => cpu.load_store_absolute(|(c, value)| c.dec(value))?,
        DEC_ABSX => cpu.load_store_absolute_x(|(c, value)| c.dec(value))?,
        DEC_ACC => cpu.set_a(cpu.a.wrapping_sub(1)),
        DEC_ZP => cpu.load_store_zeropage(|(c, value)| c.dec(value))?,
        DEC_ZPX => cpu.load_store_zeropage_x(|(c, value)| c.dec(value))?,
        DEX => cpu.set_x(cpu.x.wrapping_sub(1)),
        DEY => cpu.set_y(cpu.y.wrapping_sub(1)),
        EOR_ABS => {
            let value = cpu.load_absolute()?;
            cpu.exclusive_or(value);
        }
        EOR_ABSX => {
            let value = cpu.load_absolute_x()?;
            cpu.exclusive_or(value);
        }
        EOR_ABSY => {
            let value = cpu.load_absolute_y()?;
            cpu.exclusive_or(value);
        }
        EOR_IMM => {
            let value = cpu.load_immediate()?;
            cpu.exclusive_or(value);
        }
        EOR_IND => {
            let value = cpu.load_indirect()?;
            cpu.exclusive_or(value);
        }
        EOR_INDX => {
            let value = cpu.load_indirect_x()?;
            cpu.exclusive_or(value);
        }
        EOR_INDY => {
            let value = cpu.load_indirect_y()?;
            cpu.exclusive_or(value);
        }
        EOR_ZP => {
            let value = cpu.load_zeropage()?;
            cpu.exclusive_or(value);
        }
        EOR_ZPX => {
            let value = cpu.load_zeropage_x()?;
            cpu.exclusive_or(value);
        }
        INC_ABS => cpu.load_store_absolute(|(c, value)| c.inc(value))?,
        INC_ABSX => cpu.load_store_absolute_x(|(c, value)| c.inc(value))?,
        INC_ACC => cpu.set_a(cpu.a.wrapping_add(1)),
        INC_ZP => cpu.load_store_zeropage(|(c, value)| c.inc(value))?,
        INC_ZPX => cpu.load_store_zeropage_x(|(c, value)| c.inc(value))?,
        INX => cpu.set_x(cpu.x.wrapping_add(1)),
        INY => cpu.set_y(cpu.y.wrapping_add(1)),
        JMP_ABS => {
            let new_pc = cpu.load_absolute_address()?;
            if (cpu.pc - 3) == new_pc {
                return Err("infinite loop detected".to_string());
            }
            cpu.pc = new_pc;
        }
        JMP_ABSX => {
            let lsb_address = cpu.load_absolute_x_address()?;
            let msb_address = lsb_address + 1;
            let lsb = cpu.memory.get16(lsb_address);
            let msb = cpu.memory.get16(msb_address);
            let new_pc = utils::combine(lsb, msb, 0);
            if (cpu.pc - 3) == new_pc {
                return Err("infinite loop detected".to_string());
            }
            cpu.pc = new_pc;
        }
        JMP_IND => {
            let lsb = cpu.fetch()?;
            let msb = cpu.fetch()?;
            let new_pc = utils::combine(cpu.memory.get(lsb, msb, 0), cpu.memory.get(lsb, msb, 1), 0);
            cpu.pc = new_pc;
        }
        JSR => {
            let lsb = cpu.fetch()?;
            let msb = cpu.fetch()?;

            let target_pc = cpu.pc - 1;
            cpu.push((target_pc >> 8) as u8);
            cpu.push(target_pc as u8);

            let new_pc = utils::combine(lsb, msb, 0);
            cpu.pc = new_pc;
        }
        LDA_ABS => {
            let value = cpu.load_absolute()?;
            cpu.set_a(value)
        }
        LDA_ABSX => {
            let value = cpu.load_absolute_x()?;
            cpu.set_a(value);
        }
        LDA_ABSY => {
            let value = cpu.load_absolute_y()?;
            cpu.set_a(value);
        }
        LDA_IMM => {
            let value = cpu.load_immediate()?;
            cpu.set_a(value);
        }
        LDA_IND => {
            let value = cpu.load_indirect()?;
            cpu.set_a(value);
        }
        LDA_INDX => {
            let value = cpu.load_indirect_x()?;
            cpu.set_a(value);
        }
        LDA_INDY => {
            let value = cpu.load_indirect_y()?;
            cpu.set_a(value);
        }
        LDA_ZP => {
            let value = cpu.load_zeropage()?;
            cpu.set_a(value);
        }
        LDA_ZPX => {
            let value = cpu.load_zeropage_x()?;
            cpu.set_a(value);
        }
        LDX_ABS => {
            let value = cpu.load_absolute()?;
            cpu.set_x(value);
        }
        LDX_ABSY => {
            let value = cpu.load_absolute_y()?;
            cpu.set_x(value);
        }
        LDX_IMM => {
            let value = cpu.load_immediate()?;
            cpu.set_x(value)
        }
        LDX_ZP => {
            let value = cpu.load_zeropage()?;
            cpu.set_x(value)
        }
        LDX_ZPY => {
            let value = cpu.load_zeropage_y()?;
            cpu.set_x(value);
        }
        LDY_ABS => {
            let value = cpu.load_absolute()?;
            cpu.set_y(value);
        }
        LDY_ABSX => {
            let value = cpu.load_absolute_x()?;
            cpu.set_y(value);
        }
        LDY_IMM => {
            let value = cpu.load_immediate()?;
            cpu.set_y(value);
        }
        LDY_ZP => {
            let value = cpu.load_zeropage()?;
            cpu.set_y(value);
        }
        LDY_ZPX => {
            let value = cpu.load_zeropage_x()?;
            cpu.set_y(value);
        }
        LSR_ABS => cpu.load_store_absolute(|(c, value)| c.lsr(value))?,
        LSR_ABSX => cpu.load_store_absolute_x(|(c, value)| c.lsr(value))?,
        LSR_ACC => {
            let value = cpu.lsr(cpu.a);
            cpu.a = value; // status has already been set in asl
        }
        LSR_ZP => cpu.load_store_zeropage(|(c, value)| c.lsr(value))?,
        LSR_ZPX => cpu.load_store_zeropage_x(|(c, value)| c.lsr(value))?,
        NOP { byte_size: b } => {
            for _ in 1..*b {
                cpu.load_immediate()?;
            }
        }
        ORA_ABS => {
            let value = cpu.load_absolute()?;
            cpu.inclusive_or(value);
        }
        ORA_ABSX => {
            let value = cpu.load_absolute_x()?;
            cpu.inclusive_or(value);
        }
        ORA_ABSY => {
            let value = cpu.load_absolute_y()?;
            cpu.inclusive_or(value);
        }
        ORA_IMM => {
            let value = cpu.load_immediate()?;
            cpu.inclusive_or(value);
        }
        ORA_IND => {
            let value = cpu.load_indirect()?;
            cpu.inclusive_or(value);
        }
        ORA_INDX => {
            let value = cpu.load_indirect_x()?;
            cpu.inclusive_or(value);
        }
        ORA_INDY => {
            let value = cpu.load_indirect_y()?;
            cpu.inclusive_or(value);
        }
        ORA_ZP => {
            let value = cpu.load_zeropage()?;
            cpu.inclusive_or(value);
        }
        ORA_ZPX => {
            let value = cpu.load_zeropage_x()?;
            cpu.inclusive_or(value);
        }
        PHA => cpu.push(cpu.a as u8),
        PHP => {
            let old = cpu.b;
            cpu.b = true;
            cpu.push(cpu.get_sr());
            cpu.b = old;
        }
        PHX => cpu.push(cpu.x as u8),
        PHY => cpu.push(cpu.y as u8),
        PLA => {
            let value = cpu.pull() as i8;
            cpu.set_a(value)
        }
        PLP => {
            let old = cpu.b;
            let pulled = cpu.pull();
            cpu.set_sr(pulled);
            cpu.b = old;
        }
        PLX => {
            let value = cpu.pull() as i8;
            cpu.set_x(value);
        }
        PLY => {
            let value = cpu.pull() as i8;
            cpu.set_y(value);
        }
        RMB0 => cpu.load_store_zeropage(|(c, value)| c.reset_bit(value, 0))?,
        RMB1 => cpu.load_store_zeropage(|(c, value)| c.reset_bit(value, 1))?,
        RMB2 => cpu.load_store_zeropage(|(c, value)| c.reset_bit(value, 2))?,
        RMB3 => cpu.load_store_zeropage(|(c, value)| c.reset_bit(value, 3))?,
        RMB4 => cpu.load_store_zeropage(|(c, value)| c.reset_bit(value, 4))?,
        RMB5 => cpu.load_store_zeropage(|(c, value)| c.reset_bit(value, 5))?,
        RMB6 => cpu.load_store_zeropage(|(c, value)| c.reset_bit(value, 6))?,
        RMB7 => cpu.load_store_zeropage(|(c, value)| c.reset_bit(value, 7))?,
        ROL_ABS => cpu.load_store_absolute(|(c, value)| c.rol(value))?,
        ROL_ABSX => cpu.load_store_absolute_x(|(c, value)| c.rol(value))?,
        ROL_ACC => {
            let value = cpu.rol(cpu.a);
            cpu.a = value; // status has already been set in asl
        }
        ROL_ZP => cpu.load_store_zeropage(|(c, value)| c.rol(value))?,
        ROL_ZPX => cpu.load_store_zeropage_x(|(c, value)| c.rol(value))?,
        ROR_ABS => cpu.load_store_absolute(|(c, value)| c.ror(value))?,
        ROR_ABSX => cpu.load_store_absolute_x(|(c, value)| c.ror(value))?,
        ROR_ACC => {
            let value = cpu.ror(cpu.a);
            cpu.a = value; // status has already been set in asl
        }
        ROR_ZP => cpu.load_store_zeropage(|(c, value)| c.ror(value))?,
        ROR_ZPX => cpu.load_store_zeropage_x(|(c, value)| c.ror(value))?,
        RTI => {
            let new_sr = cpu.pull();
            cpu.set_sr(new_sr);

            let lsb = cpu.pull();
            let msb = cpu.pull();
            let new_pc = utils::combine(lsb, msb, 0);
            cpu.pc = new_pc;
        }
        RTS => {
            let lsb = cpu.pull();
            let msb = cpu.pull();
            let new_pc = utils::combine(lsb, msb, 1);
            cpu.pc = new_pc;
        }
        SBC_ABS => {
            let value = cpu.load_absolute()?;
            cpu.subtract_with_borrow(value);
        }
        SBC_ABSX => {
            let value = cpu.load_absolute_x()?;
            cpu.subtract_with_borrow(value);
        }
        SBC_ABSY => {
            let value = cpu.load_absolute_y()?;
            cpu.subtract_with_borrow(value);
        }
        SBC_IMM => {
            let value = cpu.load_immediate()?;
            cpu.subtract_with_borrow(value);
        }
        SBC_IND => {
            let value = cpu.load_indirect()?;
            cpu.subtract_with_borrow(value);
        }
        SBC_INDX => {
            let value = cpu.load_indirect_x()?;
            cpu.subtract_with_borrow(value);
        }
        SBC_INDY => {
            let value = cpu.load_indirect_y()?;
            cpu.subtract_with_borrow(value);
        }
        SBC_ZP => {
            let value = cpu.load_zeropage()?;
            cpu.subtract_with_borrow(value);
        }
        SBC_ZPX => {
            let value = cpu.load_zeropage_x()?;
            cpu.subtract_with_borrow(value);
        }
        SEC => cpu.c = true,
        SED => cpu.d = true,
        SEI => cpu.i = true,
        SMB0 => cpu.load_store_zeropage(|(c, value)| c.set_bit(value, 0))?,
        SMB1 => cpu.load_store_zeropage(|(c, value)| c.set_bit(value, 1))?,
        SMB2 => cpu.load_store_zeropage(|(c, value)| c.set_bit(value, 2))?,
        SMB3 => cpu.load_store_zeropage(|(c, value)| c.set_bit(value, 3))?,
        SMB4 => cpu.load_store_zeropage(|(c, value)| c.set_bit(value, 4))?,
        SMB5 => cpu.load_store_zeropage(|(c, value)| c.set_bit(value, 5))?,
        SMB6 => cpu.load_store_zeropage(|(c, value)| c.set_bit(value, 6))?,
        SMB7 => cpu.load_store_zeropage(|(c, value)| c.set_bit(value, 7))?,
        STA_ABS => cpu.store_absolute(cpu.a)?,
        STA_ABSX => cpu.store_absolute_x(cpu.a)?,
        STA_ABSY => cpu.store_absolute_y(cpu.a)?,
        STA_IND => cpu.store_indirect(cpu.a)?,
        STA_INDX => cpu.store_indirect_x(cpu.a)?,
        STA_INDY => cpu.store_indirect_y(cpu.a)?,
        STA_ZP => cpu.store_zeropage(cpu.a)?,
        STA_ZPX => cpu.store_zeropage_x(cpu.a)?,
        STX_ABS => cpu.store_absolute(cpu.x)?,
        STX_ZP => cpu.store_zeropage(cpu.x)?,
        STX_ZPY => cpu.store_zeropage_y(cpu.x)?,
        STY_ABS => cpu.store_absolute(cpu.y)?,
        STY_ZP => cpu.store_zeropage(cpu.y)?,
        STY_ZPX => cpu.store_zeropage_x(cpu.y)?,
        STZ_ABS => cpu.store_absolute(0)?,
        STZ_ABSX => cpu.store_absolute_x(0)?,
        STZ_ZP => cpu.store_zeropage(0)?,
        STZ_ZPX => cpu.store_zeropage_x(0)?,
        TAX => cpu.set_x(cpu.a),
        TAY => cpu.set_y(cpu.a),
        TRB_ABS => cpu.load_store_absolute(|(c, value)| c.test_and_reset_bit(value))?,
        TRB_ZP => cpu.load_store_zeropage(|(c, value)| c.test_and_reset_bit(value))?,
        TSB_ABS => cpu.load_store_absolute(|(c, value)| c.test_and_set_bit(value))?,
        TSB_ZP => cpu.load_store_zeropage(|(c, value)| c.test_and_set_bit(value))?,
        TSX => cpu.set_x(cpu.sp as i8),
        TXA => cpu.set_a(cpu.x),
        TXS => cpu.sp = cpu.x as u16,
        TYA => cpu.set_a(cpu.y),
    }
    Ok(())
}

impl CPU {
    fn add_with_carry(&mut self, summand: i8) {
        let carry: i16 = if self.c { 1 } else { 0 };
        if self.d {
            self.add_sub(from_bcd(self.a) + from_bcd(summand) + carry);
        } else {
            self.add_sub((self.a as i16) + (summand as i16) + (carry));
        }
    }

    fn add_sub(&mut self, value: i16) {
        let old = self.a as u8;
        self.a = if self.d {
            to_bcd(value.rem_euclid(100) as i8)
        } else {
            value as i8
        };

        self.v = value != (self.a as i16);
        self.c = if self.c { old >= (self.a as u8) } else { old > (self.a as u8) };
        self.set_status(self.a);
    }

    fn and(&mut self, value: i8) {
        self.a = self.a & value;
        self.set_status(self.a);
    }

    fn asl(&mut self, value: i8) -> i8 {
        self.c = value >> 7 & 1 != 0;
        let new_value = value << 1;
        self.set_status(new_value);
        new_value
    }

    fn branch(&mut self, branch: bool) -> Result<(), String> {
        let address_offset = self.load_immediate()?;
        if branch {
            if address_offset == -2 {
                return Err("infinite loop detected".to_string());
            }

            let pc = self.pc as i32;
            self.pc = (pc + (address_offset as i32)) as u16
        }
        Ok(())
    }

    fn branch_if_bit_reset(&mut self, bit_index: u8) -> Result<(), String> {
        let value = self.load_zeropage()? as u8;
        let branch = value & (1 << bit_index) == 0;
        self.branch(branch)?;
        Ok(())
    }

    fn branch_if_bit_set(&mut self, bit_index: u8) -> Result<(), String> {
        let value = self.load_zeropage()? as u8;
        let branch = value & (1 << bit_index) > 0;
        self.branch(branch)?;
        Ok(())
    }

    fn dec(&mut self, value: i8) -> i8 {
        let new_value = value.wrapping_sub(1);
        self.set_status(new_value);
        new_value
    }

    fn exclusive_or(&mut self, value: i8) {
        self.a = self.a ^ value;
        self.set_status(self.a);
    }

    fn inc(&mut self, value: i8) -> i8 {
        let new_value = value.wrapping_add(1);
        self.set_status(new_value);
        new_value
    }

    fn load_absolute(&mut self) -> Result<i8, String> {
        let address = self.load_absolute_address()?;
        Ok(self.memory.get16(address) as i8)
    }

    fn load_absolute_address(&mut self) -> Result<u16, String> {
        Ok(self.load_absolute_address_impl(0)?)
    }

    fn load_absolute_address_impl(&mut self, offset: i8) -> Result<u16, String> {
        let lsb = self.fetch()?;
        let msb = self.fetch()?;
        Ok(utils::combine(lsb, msb, offset as u8))
    }

    fn load_absolute_x(&mut self) -> Result<i8, String> {
        let address = self.load_absolute_x_address()?;
        Ok(self.memory.get16(address) as i8)
    }

    fn load_absolute_x_address(&mut self) -> Result<u16, String> {
        Ok(self.load_absolute_address_impl(self.x)?)
    }

    fn load_absolute_y(&mut self) -> Result<i8, String> {
        let lsb = self.fetch()?;
        let msb = self.fetch()?;
        Ok(self.load_absolute_y_impl(lsb, msb))
    }

    fn load_absolute_y_impl(&self, lsb: u8, msb: u8) -> i8 {
        self.memory.get(lsb, msb, self.y as u8) as i8
    }

    fn load_immediate(&mut self) -> Result<i8, String> {
        Ok(self.fetch()? as i8)
    }

    fn load_indirect(&mut self) -> Result<i8, String> {
        let address = self.load_indirect_address()?;
        Ok(self.memory.get16(address) as i8)
    }

    fn load_indirect_address(&mut self) -> Result<u16, String> {
        Ok(self.load_indirect_address_impl(0, 0)?)
    }

    fn load_indirect_address_impl(&mut self, lsb_offset: i8, combine_offset: i8) -> Result<u16, String> {
        let zp_offset = self.fetch()?;
        let lsb_address = utils::combine(zp_offset, 0, lsb_offset as u8) as u8;
        let msb_address = lsb_address + 1;
        let lsb = self.memory.get16(lsb_address as u16);
        let msb = self.memory.get16(msb_address as u16);
        Ok(utils::combine(lsb, msb, combine_offset as u8))
    }

    fn load_indirect_x(&mut self) -> Result<i8, String> {
        let address = self.load_indirect_x_address()?;
        Ok(self.memory.get16(address) as i8)
    }

    fn load_indirect_x_address(&mut self) -> Result<u16, String> {
        Ok(self.load_indirect_address_impl(self.x, 0)?)
    }

    fn load_indirect_y(&mut self) -> Result<i8, String> {
        let address = self.load_indirect_y_address()?;
        Ok(self.memory.get16(address) as i8)
    }

    fn load_indirect_y_address(&mut self) -> Result<u16, String> {
        Ok(self.load_indirect_address_impl(0, self.y)?)
    }

    fn load_zeropage(&mut self) -> Result<i8, String> {
        let zp_offset = self.fetch()?;
        Ok(self.load_zeropage_impl(zp_offset))
    }

    fn load_zeropage_impl(&self, zp_offset: u8) -> i8 {
        self.memory.get(zp_offset, 0, 0) as i8
    }

    fn load_zeropage_x(&mut self) -> Result<i8, String> {
        let zp_offset = self.fetch()?;
        Ok(self.load_zeropage_x_impl(zp_offset))
    }

    fn load_zeropage_x_impl(&self, zp_offset: u8) -> i8 {
        let address = utils::combine(zp_offset, 0, self.x as u8) as u8;
        self.memory.get16(address as u16) as i8
    }

    fn load_zeropage_y(&mut self) -> Result<i8, String> {
        let zp_offset = self.fetch()?;
        Ok(self.load_zeropage_y_impl(zp_offset))
    }

    fn load_zeropage_y_impl(&self, zp_offset: u8) -> i8 {
        let address = utils::combine(zp_offset, 0, self.y as u8) as u8;
        self.memory.get16(address as u16) as i8
    }

    fn load_store_absolute<F>(&mut self, mut consumer: F) -> Result<(), String> where F: FnMut((&mut CPU, i8)) -> i8 {
        let address = self.load_absolute_address()?;
        let value = self.memory.get16(address) as i8;
        let result = consumer((self, value));
        self.memory.set16(address, result as u8);
        Ok(())
    }

    fn load_store_absolute_x<F>(&mut self, mut consumer: F) -> Result<(), String> where F: FnMut((&mut CPU, i8)) -> i8 {
        let address = self.load_absolute_x_address()?;
        let value = self.memory.get16(address) as i8;
        let result = consumer((self, value));
        self.memory.set16(address, result as u8);
        Ok(())
    }

    fn load_store_zeropage<F>(&mut self, mut consumer: F) -> Result<(), String> where F: FnMut((&mut CPU, i8)) -> i8 {
        let zp_offset = self.fetch()?;
        let value = self.load_zeropage_impl(zp_offset);
        let result = consumer((self, value));
        self.store_zeropage_impl(zp_offset, result);
        Ok(())
    }

    fn load_store_zeropage_x<F>(&mut self, mut consumer: F) -> Result<(), String> where F: FnMut((&mut CPU, i8)) -> i8 {
        let zp_offset = self.fetch()?;
        let value = self.load_zeropage_x_impl(zp_offset);
        let result = consumer((self, value));
        self.store_zeropage_x_impl(zp_offset, result);
        Ok(())
    }

    fn lsr(&mut self, value: i8) -> i8 {
        self.c = value & 1 != 0;
        let new_value = ((value as u8) >> 1) as i8; // right shift with 0
        self.set_status(new_value);
        new_value
    }

    fn inclusive_or(&mut self, value: i8) {
        self.a = self.a | value;
        self.set_status(self.a);
    }

    fn pull(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.memory.get(self.sp as u8, 1, 0)
    }

    fn push(&mut self, value: u8) {
        self.memory.set(self.sp as u8, 1, 0, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn reset_bit(&mut self, value: i8, bit_index: u8) -> i8 {
        value & !(1 << bit_index)
    }

    fn rol(&mut self, value: i8) -> i8 {
        let old_c = self.c;
        self.c = value >> 7 & 1 != 0;

        let carry: i8 = if old_c { 1 } else { 0 };
        let new_value = (value << 1) + carry;
        self.set_status(new_value);
        new_value
    }

    fn ror(&mut self, value: i8) -> i8 {
        let old_c = self.c;
        self.c = value & 1 != 0;

        let carry: i8 = if old_c { 1 << 7 } else { 0 };
        let new_value = ((value as u8) >> 1) as i8 + carry;
        self.set_status(new_value);
        new_value
    }

    fn set_a(&mut self, value: i8) {
        self.a = value;
        self.set_status(self.a);
    }

    fn set_bit(&mut self, value: i8, bit_index: u8) -> i8 {
        value | (1 << bit_index)
    }

    fn set_x(&mut self, value: i8) {
        self.x = value;
        self.set_status(self.x);
    }

    fn set_y(&mut self, value: i8) {
        self.y = value;
        self.set_status(self.y);
    }

    fn set_sr(&mut self, value: u8) {
        self.n = value >> 7 & 1 != 0;
        self.v = value >> 6 & 1 != 0;
        self.b = value >> 4 & 1 != 0;
        self.d = value >> 3 & 1 != 0;
        self.i = value >> 2 & 1 != 0;
        self.z = value >> 1 & 1 != 0;
        self.c = value & 1 != 0;
    }

    fn set_status(&mut self, status: i8) {
        self.n = status < 0;
        self.z = status == 0;
    }

    fn set_status_compare(&mut self, status: i8) {
        self.set_status(status);
        self.c = status >= 0;
    }

    fn store_absolute(&mut self, value: i8) -> Result<(), String> {
        let lsb = self.fetch()?;
        let msb = self.fetch()?;
        self.store_absolute_impl(lsb, msb, value);
        Ok(())
    }

    fn store_absolute_impl(&mut self, lsb: u8, msb: u8, value: i8) {
        self.memory.set(lsb, msb, 0, value as u8);
    }

    fn store_absolute_x(&mut self, value: i8) -> Result<(), String> {
        let lsb = self.fetch()?;
        let msb = self.fetch()?;
        self.store_absolute_x_impl(lsb, msb, value);
        Ok(())
    }

    fn store_absolute_x_impl(&mut self, lsb: u8, msb: u8, value: i8) {
        self.memory.set(lsb, msb, self.x as u8, value as u8)
    }

    fn store_absolute_y(&mut self, value: i8) -> Result<(), String> {
        let lsb = self.fetch()?;
        let msb = self.fetch()?;
        self.store_absolute_y_impl(lsb, msb, value);
        Ok(())
    }

    fn store_absolute_y_impl(&mut self, lsb: u8, msb: u8, value: i8) {
        self.memory.set(lsb, msb, self.y as u8, value as u8);
    }

    fn store_indirect(&mut self, value: i8) -> Result<(), String> {
        let address = self.load_indirect_address()?;
        self.memory.set16(address, value as u8);
        Ok(())
    }

    fn store_indirect_x(&mut self, value: i8) -> Result<(), String> {
        let address = self.load_indirect_x_address()?;
        self.memory.set16(address, value as u8);
        Ok(())
    }

    fn store_indirect_y(&mut self, value: i8) -> Result<(), String> {
        let address = self.load_indirect_y_address()?;
        self.memory.set16(address, value as u8);
        Ok(())
    }

    fn store_zeropage(&mut self, value: i8) -> Result<(), String> {
        let zp_offset = self.fetch()?;
        self.store_zeropage_impl(zp_offset, value);
        Ok(())
    }

    fn store_zeropage_impl(&mut self, zp_offset: u8, value: i8) {
        self.memory.set(zp_offset, 0, 0, value as u8);
    }

    fn store_zeropage_x(&mut self, value: i8) -> Result<(), String> {
        let zp_offset = self.fetch()?;
        self.store_zeropage_x_impl(zp_offset, value);
        Ok(())
    }

    fn store_zeropage_x_impl(&mut self, zp_offset: u8, value: i8) {
        let address = utils::combine(zp_offset, 0, self.x as u8) as u8;
        self.memory.set16(address as u16, value as u8);
    }

    fn store_zeropage_y(&mut self, value: i8) -> Result<(), String> {
        let zp_offset = self.fetch()?;
        self.store_zeropage_y_impl(zp_offset, value);
        Ok(())
    }

    fn store_zeropage_y_impl(&mut self, zp_offset: u8, value: i8) {
        let address = utils::combine(zp_offset, 0, self.y as u8) as u8;
        self.memory.set16(address as u16, value as u8);
    }

    fn subtract_with_borrow(&mut self, subtrahend: i8) {
        let borrow: i16 = if self.c { 0 } else { 1 };
        if self.d {
            self.add_sub(from_bcd(self.a) - from_bcd(subtrahend) - borrow);
        } else {
            self.add_sub((self.a as i16) - (subtrahend as i16) - borrow);
        }
    }

    fn test_bit(&mut self, value: i8, set_nv: bool) {
        self.z = self.a & value == 0;
        if set_nv {
            self.n = (value >> 7) & 1 != 0;
            self.v = (value >> 6) & 1 != 0;
        }
    }

    fn test_and_reset_bit(&mut self, value: i8) -> i8 {
        self.z = self.a & value == 0;
        let mask = !self.a;
        value & mask
    }

    fn test_and_set_bit(&mut self, value: i8) -> i8 {
        self.z = self.a & value == 0;
        let mask = self.a;
        value | mask
    }
}

fn from_bcd(value: i8) -> i16 {
    let mask = 0b00001111u8;
    let value = value as u8;

    let lower_bits = value & mask;
    let upper_bits = (value >> 4) & mask;

    (upper_bits * 10 + lower_bits) as i16
}

#[allow(clippy::wrong_self_convention)]
fn to_bcd(value: i8) -> i8 {
    let value = value as u8;

    let lower = value % 10;
    let upper = value / 10;

    ((upper << 4) + lower) as i8
}
