//! Concrete (not wrapper) types representing bitfield values.

#![allow(non_camel_case_types)]

use std::fmt;

/// Condition codes.
#[derive(Debug)]
pub enum Cond {
    Eq, Ne, Cs, Cc, Mi, Pl, Vs, Vc, Hi, Ls, Ge, Lt, Gt, Le, Al, Un
}
impl Cond {
    pub fn from_u32(x: u32) -> Self {
        match x { 
            0b0000 => Cond::Eq,
            0b0001 => Cond::Ne,
            0b0010 => Cond::Cs,
            0b0011 => Cond::Cc,
            0b0100 => Cond::Mi,
            0b0101 => Cond::Pl,
            0b0110 => Cond::Vs,
            0b0111 => Cond::Vc,
            0b1000 => Cond::Hi,
            0b1001 => Cond::Ls,
            0b1010 => Cond::Ge,
            0b1011 => Cond::Lt,
            0b1100 => Cond::Gt,
            0b1101 => Cond::Le,
            0b1110 => Cond::Al,
            0b1111 => Cond::Un,
            _ => unreachable!(),
        }
    }
}
impl fmt::Display for Cond {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cond::Eq => write!(f, "eq"),
            Cond::Ne => write!(f, "ne"),
            Cond::Cs => write!(f, "cs"),
            Cond::Cc => write!(f, "cc"),
            Cond::Mi => write!(f, "mi"),
            Cond::Pl => write!(f, "pl"),
            Cond::Vs => write!(f, "vs"),
            Cond::Vc => write!(f, "vc"),
            Cond::Hi => write!(f, "hi"),
            Cond::Ls => write!(f, "ls"),
            Cond::Ge => write!(f, "ge"),
            Cond::Lt => write!(f, "lt"),
            Cond::Gt => write!(f, "gt"),
            Cond::Le => write!(f, "le"),
            Cond::Al => write!(f, ""),
            Cond::Un => write!(f, ""),
            _ => unreachable!(),
        }
    }
}




/// Data-processing opcodes.
#[derive(Debug)]
pub enum Opcode {
    None, 
    And, Eor, Sub, Rsb, Add, Adc, Sbc, Rsc, 
    Tst, Teq, Cmp, Cmn, Orr, Mov, Bic, Mvn,
}
impl Opcode {
    pub fn from_u32(x: u32) -> Self {
        match x { 
            0b0000 => Opcode::And,
            0b0001 => Opcode::Eor,
            0b0010 => Opcode::Sub,
            0b0011 => Opcode::Rsb,
            0b0100 => Opcode::Add,
            0b0101 => Opcode::Adc,
            0b0110 => Opcode::Sbc,
            0b0111 => Opcode::Rsc,
            0b1000 => Opcode::Tst,
            0b1001 => Opcode::Teq,
            0b1010 => Opcode::Cmp,
            0b1011 => Opcode::Cmn,
            0b1100 => Opcode::Orr,
            0b1101 => Opcode::Mov,
            0b1110 => Opcode::Bic,
            0b1111 => Opcode::Mvn,
            _ => unreachable!(),
        }
    }
}
impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::And => write!(f, "and"),
            Opcode::Eor => write!(f, "eor"),
            Opcode::Sub => write!(f, "sub"),
            Opcode::Rsb => write!(f, "rsb"),
            Opcode::Add => write!(f, "add"),
            Opcode::Adc => write!(f, "adc"),
            Opcode::Sbc => write!(f, "sbc"),
            Opcode::Rsc => write!(f, "rsc"),
            Opcode::Tst => write!(f, "tst"),
            Opcode::Teq => write!(f, "teq"),
            Opcode::Cmp => write!(f, "cmp"),
            Opcode::Cmn => write!(f, "cmn"),
            Opcode::Orr => write!(f, "orr"),
            Opcode::Mov => write!(f, "mov"),
            Opcode::Bic => write!(f, "bic"),
            Opcode::Mvn => write!(f, "mvn"),
            _ => unreachable!(),
        }
    }
}


/// General-purpose registers.
#[derive(Debug)]
pub enum Register {
    r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, ip, sp, lr, pc
}
impl Register {
    pub fn from_u32(x: u32) -> Self { 
        match x {
            0 => Register::r0,
            1 => Register::r1,
            2 => Register::r2,
            3 => Register::r3,
            4 => Register::r4,
            5 => Register::r5,
            6 => Register::r6,
            7 => Register::r7,
            8 => Register::r8,
            9 => Register::r9,
            10=> Register::r10,
            11 => Register::r11,
            12 => Register::ip,
            13 => Register::sp,
            14 => Register::lr,
            15 => Register::pc,
            _ => unreachable!(),
        }
    }
}
impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Register::r0  => write!(f, "r0"),
            Register::r1  => write!(f, "r1"),
            Register::r2  => write!(f, "r2"),
            Register::r3  => write!(f, "r3"),
            Register::r4  => write!(f, "r4"),
            Register::r5  => write!(f, "r5"),
            Register::r6  => write!(f, "r6"),
            Register::r7  => write!(f, "r7"),
            Register::r8  => write!(f, "r8"),
            Register::r9  => write!(f, "r9"),
            Register::r10 => write!(f, "r10"),
            Register::r11 => write!(f, "r11"),
            Register::ip  => write!(f, "ip"),
            Register::sp  => write!(f, "sp"),
            Register::lr  => write!(f, "lr"),
            Register::pc  => write!(f, "pc"),
            _ => unreachable!(),
        }
    }
}


/// Representing different kinds of shifter operations.
pub enum ShifterType { Lsl, Lsr, Asr, Ror, }
impl fmt::Display for ShifterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ShifterType::Lsl => write!(f, "lsl"),
            ShifterType::Lsr => write!(f, "lsr"),
            ShifterType::Asr => write!(f, "asr"),
            ShifterType::Ror => write!(f, "ror"),
        }
    }
}
impl ShifterType {
    pub fn from_u32(x: u32) -> Self {
        match x {
            0b00 => ShifterType::Lsl,
            0b01 => ShifterType::Lsr,
            0b10 => ShifterType::Asr,
            0b11 => ShifterType::Ror,
            _ => unreachable!(),
        }
    }

    pub fn compute(&self, val: u32, rot: u32) -> (u32, Option<bool>) {
        if rot == 0 { return (val, None); }

        return match self { 
            ShifterType::Lsl => {(
                (val << rot), 
                Some((1 << (31 - rot) & val) != 0)
            )},
            ShifterType::Lsr => {(
                (val << rot), 
                Some((1 << (rot - 1) & val) != 0)
            )},
            ShifterType::Asr => {(
                ((val as i32) >> rot) as u32, 
                Some((1 << (rot - 1) & val) != 0)
            )},
            ShifterType::Ror => {(
                val.rotate_right(rot), 
                Some((val & 0x8000_0000) != 0)
            )},
        };
    }
}

/// Representing different coprocessor numbers.
#[derive(Debug)]
pub enum CoprocNumber {
    p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15
}
impl CoprocNumber {
    pub fn from_u32(x: u32) -> Self {
        match x {
            0b0000 => CoprocNumber::p0,
            0b0001 => CoprocNumber::p1,
            0b0010 => CoprocNumber::p2,
            0b0011 => CoprocNumber::p3,
            0b0100 => CoprocNumber::p4,
            0b0101 => CoprocNumber::p5,
            0b0110 => CoprocNumber::p6,
            0b0111 => CoprocNumber::p7,
            0b1000 => CoprocNumber::p8,
            0b1001 => CoprocNumber::p9,
            0b1010 => CoprocNumber::p10,
            0b1011 => CoprocNumber::p11,
            0b1100 => CoprocNumber::p12,
            0b1101 => CoprocNumber::p13,
            0b1110 => CoprocNumber::p14,
            0b1111 => CoprocNumber::p15,
            _ => unreachable!(),
        }
    }
}
impl fmt::Display for CoprocNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CoprocNumber::p0 => write!(f, "p0"),
            CoprocNumber::p1 => write!(f, "p1"),
            CoprocNumber::p2 => write!(f, "p2"),
            CoprocNumber::p3 => write!(f, "p3"),
            CoprocNumber::p4 => write!(f, "p4"),
            CoprocNumber::p5 => write!(f, "p5"),
            CoprocNumber::p6 => write!(f, "p6"),
            CoprocNumber::p7 => write!(f, "p7"),
            CoprocNumber::p8 => write!(f, "p8"),
            CoprocNumber::p9 => write!(f, "p9"),
            CoprocNumber::p10 => write!(f, "p10"),
            CoprocNumber::p11 => write!(f, "p11"),
            CoprocNumber::p12 => write!(f, "p12"),
            CoprocNumber::p13 => write!(f, "p13"),
            CoprocNumber::p14 => write!(f, "p14"),
            CoprocNumber::p15 => write!(f, "p15"),
        }
    }
}


/// Representing different coprocessor registers.
#[derive(Debug)]
pub enum CoprocRegister {
    cr0, cr1, cr2, cr3, cr4, cr5, cr6, cr7, 
    cr8, cr9, cr10, cr11, cr12, cr13, cr14, cr15,
}
impl CoprocRegister {
    pub fn from_u32(x: u32) -> Self {
        match x {
            0b0000 => CoprocRegister::cr0,
            0b0001 => CoprocRegister::cr1,
            0b0010 => CoprocRegister::cr2,
            0b0011 => CoprocRegister::cr3,
            0b0100 => CoprocRegister::cr4,
            0b0101 => CoprocRegister::cr5,
            0b0110 => CoprocRegister::cr6,
            0b0111 => CoprocRegister::cr7,
            0b1000 => CoprocRegister::cr8,
            0b1001 => CoprocRegister::cr9,
            0b1010 => CoprocRegister::cr10,
            0b1011 => CoprocRegister::cr11,
            0b1100 => CoprocRegister::cr12,
            0b1101 => CoprocRegister::cr13,
            0b1110 => CoprocRegister::cr14,
            0b1111 => CoprocRegister::cr15,
            _ => unreachable!(),
        }
    }
}
impl fmt::Display for CoprocRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CoprocRegister::cr0 => write!(f, "cr0"),
            CoprocRegister::cr1 => write!(f, "cr1"),
            CoprocRegister::cr2 => write!(f, "cr2"),
            CoprocRegister::cr3 => write!(f, "cr3"),
            CoprocRegister::cr4 => write!(f, "cr4"),
            CoprocRegister::cr5 => write!(f, "cr5"),
            CoprocRegister::cr6 => write!(f, "cr6"),
            CoprocRegister::cr7 => write!(f, "cr7"),
            CoprocRegister::cr8 => write!(f, "cr8"),
            CoprocRegister::cr9 => write!(f, "cr9"),
            CoprocRegister::cr10 => write!(f, "cr10"),
            CoprocRegister::cr11 => write!(f, "cr11"),
            CoprocRegister::cr12 => write!(f, "cr12"),
            CoprocRegister::cr13 => write!(f, "cr13"),
            CoprocRegister::cr14 => write!(f, "cr14"),
            CoprocRegister::cr15 => write!(f, "cr15"),
        }
    }
}


