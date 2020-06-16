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


pub fn blx_imm(op: &BranchBf, offset: u32) -> String { 
    let imm24 = sign_extend(op.imm24() as i32, 24) << 2;
    //let dest = (offset as i64) + (imm24 as i64) + 8;
    format!("blx{}\t {}",
        Cond::from_u32(op.cond()),
        imm24 as i64
    )
}
pub fn blx_reg(op: &BranchBf) -> String { 
    format!("blx\t {}", Register::from_u32(op.rm()))
}

pub fn bx(op: &BxBf) -> String { format!("bx{}\t {}", 
    Cond::from_u32(op.cond()), Register::from_u32(op.rm()))
}


pub fn bl(op: &BranchBf, offset: u32) -> String {
    let imm24 = sign_extend(op.imm24() as i32, 24) << 2;
    //let dest = (offset as i64) + (imm24 as i64) + 8;
    format!("bl{}\t {}",
        Cond::from_u32(op.cond()),
        imm24 as i64
    )
}

pub fn b(op: &BranchBf, offset: u32) -> String {
    let imm24 = sign_extend(op.imm24() as i32, 24) << 2;
    //let dest = (offset as i64) + (imm24 as i64) + 8;
    format!("b{}\t {}",
        Cond::from_u32(op.cond()),
        imm24 as i64
    )
}


