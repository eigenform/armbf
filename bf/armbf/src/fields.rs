//! Concrete (not wrapper) types representing bitfield values.

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
    None, And, Eor, Sub, Rsb, Add, Adc, Sbc, Rsc, Tst, Teq, Cmp, Cmn, Orr, Mov, Bic, Mvn,
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


/// Registers.
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

   fn regname(x: u32) -> &'static str {
        match x {
            0 => "r0",
            1 => "r1",
            2 => "r2",
            3 => "r3",
            4 => "r4",
            5 => "r5",
            6 => "r6",
            7 => "r7",
            8 => "r8",
            9 => "r9",
            10=> "r10",
            11 => "r11",
            12 => "ip",
            13 => "sp",
            14 => "lr",
            15 => "pc",
            _ => unreachable!(),
        }
    }


