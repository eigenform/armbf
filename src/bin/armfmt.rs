//! Functions returning strings with formatted ARM instructions.

use armbf::newtype::*;
use armbf::traits::*;
use armbf::fields::*;

/// Sign extend to some number of bits
#[inline(always)]
pub fn sign_extend(x: i32, bits: i32) -> i32 {
    if ( (x >> (bits - 1)) & 1) != 0 {
        return x | !0 << bits
    }
    x
}


// ----------------------------------------------------------------------------
// Control instructions

pub fn swi(op: &SwiBf) -> String { format!("svc\t 0x{:08x}", op.imm24()) }

pub fn bkpt(op: &BkptBf) -> String { 
    format!("bkpt\t 0x{:04x}", 
        ((op.imm12_hi() << 4) | op.imm4()) as u16
    )
}


pub fn mrs(op: &StatusBf) -> String {
    let sr_name = if op.r() { "SPSR" } else { "CPSR" };
    format!("mrs{}\t {}, {}",
        Cond::from_u32(op.cond()), 
        Register::from_u32(op.rd()),
        sr_name
    )
}


// ----------------------------------------------------------------------------
// Saturated add/sub instructions

pub fn qadd(op: &SatBf) -> String { format!("qadd{}\t {}, {}, {}",
        Cond::from_u32(op.cond()), 
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()), 
        Register::from_u32(op.rn()),
    )
}
pub fn qdadd(op: &SatBf) -> String { format!("qdadd{}\t {}, {}, {}",
        Cond::from_u32(op.cond()), 
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()), 
        Register::from_u32(op.rn()),
    )
}
pub fn qsub(op: &SatBf) -> String { format!("qsub{}\t {}, {}, {}",
        Cond::from_u32(op.cond()), 
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()), 
        Register::from_u32(op.rn()),
    )
}
pub fn qdsub(op: &SatBf) -> String { format!("qdsub{}\t {}, {}, {}",
        Cond::from_u32(op.cond()), 
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()), 
        Register::from_u32(op.rn()),
    )
}


// ----------------------------------------------------------------------------
// Load/store instructions

pub fn ls_imm(op: &LsImmBf) -> String {
    let name = match op.l() { true => "ldr", false => "str", };
    let width = match op.b() { true => "b", false => "", };
    format!("{}{}\t {}, [{}, #{}]", 
        name, width, 
        Register::from_u32(op.rd()),
        Register::from_u32(op.rn()),
        op.imm12(),
    )
}

pub fn ls_multi(op: &LsMultiBf) -> String {
    let mut reglist_str = std::string::String::new();
    let rn = Register::from_u32(op.rn());
    let wb = if op.w() { "!" } else { "" };
    let name = if op.l() { "ldm" } else { "stm" };

    for idx in 0..16 {
        if (op.reglist() & (1 << idx)) != 0 {
            reglist_str.push_str( format!("{}, ", 
                    Register::from_u32(idx)).as_ref()
            );
        }
    }
    reglist_str.truncate(reglist_str.len() - 2);
    format!("{} {}{}, {{{}}}", name, rn, wb, reglist_str)
}

pub fn swp(op: &SwpBf) -> String { 
    let name = if op.b() { "swpb" } else { "swp" };
    format!("{}\t {}, {}, [{}]", 
        name,
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rn()),
    )
}

pub fn ls_halfword_imm(op: &LsHalfImmBf) -> String {
    let name = if op.l() { "ldrh" } else { "strh" };
    let imm = if op.u() {
        ((op.off_hi() << 4) | op.off_lo()) as i32
    } else {
        (((op.off_hi() << 4) | op.off_lo()) as i32).wrapping_neg()
    };
    format!("{}{}\t {}, [{}, #{}]",
        name,
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd()),
        Register::from_u32(op.rn()),
        imm,
    )
}

pub fn ls_halfword_reg(op: &LsHalfRegBf) -> String {
    let name = if op.l() { "ldrh" } else { "strh" };
    format!("{}{}\t {}, [{}, {}]",
        name,
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd()),
        Register::from_u32(op.rn()),
        Register::from_u32(op.rm()),
    )
}

pub fn clz(op: &ClzBf) -> String {
    format!("clz{}\t {}, {}",
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()),
    )
}


// ----------------------------------------------------------------------------
// Data processing instructions

pub fn dp_rot_imm(op: &DpRotImmBf) -> String { 
    let opcd = Opcode::from_u32(op.opcd());
    let cond = Cond::from_u32(op.cond());
    let rn = Register::from_u32(op.rn());
    let rd = Register::from_u32(op.rd());
    let imm8 = op.imm8();

    return match opcd {
        Opcode::Cmp => format!("{}{}\t {}, #{}", opcd, cond, rn, imm8),
        Opcode::Cmn => format!("{}{}\t {}, #{}", opcd, cond, rn, imm8),
        Opcode::Mov => format!("{}{}\t {}, #{}", opcd, cond, rd, imm8),
        Opcode::Mvn => format!("{}{}\t {}, #{}", opcd, cond, rd, imm8),
        Opcode::Teq => format!("{}{}\t {}, #{}", opcd, cond, rn, imm8),
        Opcode::Tst => format!("{}{}\t {}, #{}", opcd, cond, rn, imm8),
        _ => format!("{}{}\t {}, {}, #{}", opcd, cond, rd, rn, imm8),
    };

}

pub fn dp_shift_imm(op: &DpShiftImmBf) -> String { 
    let opcd = Opcode::from_u32(op.opcd());
    let cond = Cond::from_u32(op.cond());
    let rn = Register::from_u32(op.rn());
    let rd = Register::from_u32(op.rd());
    let rm = Register::from_u32(op.rm());
    let shift_imm = op.shift_imm();
    let shift_type = ShifterType::from_u32(op.shift());

    match opcd {
        Opcode::Cmn => { 
            if shift_imm == 0 {
                return format!("{}{}\t {}, {}", opcd, cond, rn, rm);
            }
            format!("{}{}\t {}, {}", opcd, cond, rn, 
                format!("{}, {} #{}", rm, shift_type, shift_imm)
            )
        },
        Opcode::Cmp => { 
            if shift_imm == 0 {
                return format!("{}{}\t {}, {}", opcd, cond, rn, rm);
            }
            format!("{}{}\t {}, {}", opcd, cond, rn, 
                format!("{}, {} #{}", rm, shift_type, shift_imm)
            )
        },

        Opcode::Mov => { 
            if shift_imm == 0 {
                return format!("{}{}\t {}, {}", opcd, cond, rd, rm);
            }
            format!("{}{}\t {}, {}, #{}", shift_type, cond, rd, 
                rm, shift_imm
            )
        },

        Opcode::Mvn => { 
            if shift_imm ==0 {
                return format!("{}{}\t {}, {}", opcd, cond, rd, rm);
            }
            format!("{}{}\t {}, {}, {} #{}", opcd, cond, rd, 
                rm, shift_type, shift_imm
            )
        },

        Opcode::Teq => { 
            if shift_imm == 0 {
                return format!("{}{}\t {}, {}", opcd, cond, rn, rm);
            }
            format!("{}{}\t {}, {}", opcd, cond, rn, 
                format!("{}, {} #{}", rm, shift_type, shift_imm)
            )
        },
        Opcode::Tst => { 
            if shift_imm == 0 {
                return format!("{}{}\t {}, {}", opcd, cond, rn, rm);
            }
            format!("{}{}\t {}, {}", opcd, cond, rn, 
                format!("{}, {} #{}", rm, shift_type, shift_imm)
            )
        },
        _ => { 
            if shift_imm == 0 {
                return format!("{}{}\t {}, {}, {}", opcd, cond, rd, rn, rm);
            }
            format!("{}{}\t {}, {}, {} {} #{}", opcd, cond, rd, rn, 
                rm, shift_type, shift_imm
            )
        },
    }
}

pub fn dp_shift_reg(op: &DpShiftRegBf) -> String { 
    let opcd = Opcode::from_u32(op.opcd());
    let cond = Cond::from_u32(op.cond());
    let rn = Register::from_u32(op.rn());
    let rd = Register::from_u32(op.rd());
    let rm = Register::from_u32(op.rm());
    let rs = Register::from_u32(op.rs());
    let shift_type = ShifterType::from_u32(op.shift());

    match opcd {
        Opcode::Cmn => { format!("{}{}\t {}, {}", opcd, cond, rn, 
            format!("{}, {} {}", rm, shift_type, rs))
        },
        Opcode::Cmp => { format!("{}{}\t {}, {}", opcd, cond, rn, 
            format!("{}, {} {}", rm, shift_type, rs))
        },

        Opcode::Mov => { format!("{}{}\t {}, {}, {}", 
            shift_type, cond, rd, rm, rs)
        },

        Opcode::Mvn => { format!("{}{}\t {}, {}, {} {}", 
            opcd, cond, rd, rm, shift_type, rs)
        },

        Opcode::Teq => { format!("{}{}\t {}, {}", opcd, cond, rn, 
            format!("{}, {} {}", rm, shift_type, rs))
        },
        Opcode::Tst => { format!("{}{}\t {}, {}", opcd, cond, rn, 
            format!("{}, {} {}", rm, shift_type, rs))
        },
        _ => { format!("{}{}\t {}, {}, {} {} {}", opcd, cond, rd, rn, 
            rm, shift_type, rs)
        },
    }
}


// ----------------------------------------------------------------------------
// Branching instructions

pub fn blx_imm(op: &BranchBf, offset: u32) -> String { 
    let imm24 = sign_extend(op.imm24() as i32, 24) << 2;
    let dest = (offset as i64) + (imm24 as i64) + 8;
    format!("blx{}\t {:04x}",
        Cond::from_u32(op.cond()),
        dest
    )
}
pub fn blx_reg(op: &BranchBf) -> String { 
    format!("blx\t {}", Register::from_u32(op.rm()))
}

pub fn bx(op: &BxBf) -> String { format!("bx{}\t {}", 
    Cond::from_u32(op.cond()), Register::from_u32(op.rm()))
}
pub fn branch(op: &BranchBf, offset: u32) -> String {
    let name = if op.link() { "bl" } else { "b" };
    let imm24 = sign_extend(op.imm24() as i32, 24) << 2;
    let dest = (offset as i64) + (imm24 as i64) + 8;
    format!("{}{}\t {:04x}",
        name, 
        Cond::from_u32(op.cond()),
        dest,
    )
}


// ----------------------------------------------------------------------------
// Multiply instructions (extended)

pub fn smla_xy(op: &MulBf) -> String {
    let xy = match (op.x(), op.y()) {
        (false, false) => "bb", (false, true) => "bt",
        (true, false) => "tb", (true, true) => "tt",
    };
    format!("slma{}{}\t {}, {}, {}, {}", xy,
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd_alt()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
        Register::from_u32(op.rn_alt()),
    )
}

pub fn smlal_xy(op: &MulBf) -> String {
    let xy = match (op.x(), op.y()) {
        (false, false) => "bb", (false, true) => "bt",
        (true, false) => "tb", (true, true) => "tt",
    };
    format!("slmal{}{} {}, {}, {}, {}", xy,
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rn_alt()),
        Register::from_u32(op.rd_alt()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
    )
}

pub fn smul_xy(op: &MulBf) -> String {
    let xy = match (op.x(), op.y()) {
        (false, false) => "bb", (false, true) => "bt",
        (true, false) => "tb", (true, true) => "tt",
    };
    format!("smul{}{}  {}, {}, {}", xy,
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd_alt()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
    )
}

pub fn smlaw_y(op: &MulBf) -> String {
    let y = if op.y() { "t" } else { "b" };
    format!("slmaw{}{}  {}, {}, {}, {}", y,
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd_alt()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
        Register::from_u32(op.rn_alt()),
    )
}

pub fn smulw_y(op: &MulBf) -> String {
    let y = if op.y() { "t" } else { "b" };
    format!("smulw{}{}  {}, {}, {}", y,
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd_alt()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
    )
}


// ----------------------------------------------------------------------------
// Multiply instructions

pub fn mul(op: &MulBf) -> String {
    format!("mul{}\t {}, {}, {}",
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd_alt()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
    )
}
pub fn mla(op: &MulBf) -> String {
    format!("mul{}\t {}, {}, {}, {}",
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd_alt()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
        Register::from_u32(op.rn_alt()),
    )
}

pub fn umull(op: &MulBf) -> String {
    format!("umull{}\t {}, {}, {}, {}",
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd_lo()),
        Register::from_u32(op.rd_hi()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
    )
}

pub fn umlal(op: &MulBf) -> String {
    format!("umlal{}\t {}, {}, {}, {}",
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd_lo()),
        Register::from_u32(op.rd_hi()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
    )
}

pub fn smlal(op: &MulBf) -> String {
    format!("smlal{}\t {}, {}, {}, {}",
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd_lo()),
        Register::from_u32(op.rd_hi()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
    )
}

pub fn smull(op: &MulBf) -> String {
    format!("smull{}\t {}, {}, {}, {}",
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd_lo()),
        Register::from_u32(op.rd_hi()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
    )
}


// ----------------------------------------------------------------------------
// Coprocessor instructions

pub fn mrc(op: &CoprocBf) -> String {
    format!("mrc{}\t {}, {}, {}, {}, {}, {{{}}}",
        Cond::from_u32(op.cond()),
        CoprocNumber::from_u32(op.cp_num()),
        op.opcd1(),
        Register::from_u32(op.crd()),
        CoprocRegister::from_u32(op.crn()),
        CoprocRegister::from_u32(op.crm()),
        op.opcd2(),
    )
}

pub fn mcr(op: &CoprocBf) -> String {
    format!("mcr{}\t {}, {}, {}, {}, {}, {{{}}}",
        Cond::from_u32(op.cond()),
        CoprocNumber::from_u32(op.cp_num()),
        op.opcd1(),
        Register::from_u32(op.crd()),
        CoprocRegister::from_u32(op.crn()),
        CoprocRegister::from_u32(op.crm()),
        op.opcd2(),
    )
}


