//! Traits implemented on newtypes for representing bitfields.
//!
//! These traits describe some set of bitfields that may or may not belong to
//! a particular group/class/type of ARM instructions. All of the functions in 
//! these traits have some corresponding macro in the armbf_prim crate which 
//! perform some operation on a u32.

/* 
 * The following are traits representing bitfields on ARM instructions.
 */


/// Accessors common to all instructions.
pub trait InstBits {
    fn cond(&self) -> u32;
    fn group(&self) -> u32;
}

/// Accessors common to data processing instructions.
pub trait DpBits {
    fn opcd(&self) -> u32;
    fn s(&self) -> bool;
}

/// Accessors common to load/store multiple instructions.
pub trait LsMultiBits {
    fn s(&self) -> bool;
    fn reglist(&self) -> u32;
}

/// Accessors common to multiply instructions.
pub trait MultiplyBits {
    fn rd_hi(&self) -> u32;
    fn rd_lo(&self) -> u32;
    fn a(&self) -> bool;
    fn un(&self) -> bool;
    fn x(&self) -> bool;
    fn y(&self) -> bool;
}

/// Accessors common to load/store instructions.
pub trait LsBits {
    fn p(&self) -> bool;
    fn u(&self) -> bool;
    fn b(&self) -> bool;
    fn w(&self) -> bool;
    fn l(&self) -> bool;
}

/// Accessors for immediates.
pub trait ImmBits {
    fn imm4(&self) -> u32;
    fn imm8(&self) -> u32;
    fn imm12(&self) -> u32;
    fn imm12_hi(&self) -> u32;
    fn imm24(&self) -> u32;
    fn off_hi(&self) -> u32;
    fn off_lo(&self) -> u32;
}

/// Accessors for instructions that modify the status registers.
pub trait SrBits {
    fn field_mask(&self) -> u32;
    fn r(&self) -> bool;
}

/// Accessors in branching instructions.
pub trait BranchBits { fn link(&self) -> bool; }

/// Accessors for rotate instructions.
pub trait RotBits { fn rot_imm(&self) -> u32; }

/// Accessors for shifter instructions.
pub trait ShiftBits {
    fn shift_imm(&self) -> u32;
    fn shift(&self) -> u32;
}

/// Accessors common to coprocessor instructions.
pub trait CoprocBits {
    fn opcd1(&self) -> u32;
    fn opcd1_rt(&self) -> u32;
    fn cp_num(&self) -> u32;
    fn opcd2(&self) -> u32;
    fn crn(&self) -> u32;
    fn crd(&self) -> u32;
    fn crm(&self) -> u32;
}

/// Accessors for common register fields.
pub trait RegBits {
    fn rn(&self) -> u32;
    fn rd(&self) -> u32;
    fn rm(&self) -> u32;
    fn rs(&self) -> u32;
}


/* 
 * The following are traits representing bitfields on Thumb instructions.
 */


/// Thumb data-processing, format 1
pub trait DpFmt1Bits {
    fn rd(&self) -> u16;
    fn rn(&self) -> u16;
    fn rm(&self) -> u16;
    fn op1(&self) -> bool;
}

/// Thumb data-processing, format 2
pub trait DpFmt2Bits {
    fn rd(&self) -> u16;
    fn rn(&self) -> u16;
    fn imm3(&self) -> u16;
    fn op2(&self) -> bool;
}

/// Thumb data-processing, format 3
pub trait DpFmt3Bits {
    fn imm8(&self) -> u16;
    fn rd(&self) -> u16;
    fn rn(&self) -> u16;
    fn op3(&self) -> u16;
}

/// Thumb data-processing, format 4
pub trait DpFmt4Bits {
    fn rd(&self) -> u16;
    fn rm(&self) -> u16;
    fn shift_imm(&self) -> u16;
    fn op4(&self) -> u16;
}

/// Thumb data-processing, format 5
pub trait DpFmt5Bits {
    fn rd(&self) -> u16;
    fn rn(&self) -> u16;
    fn rm(&self) -> u16;
    fn rs(&self) -> u16;
    fn op5(&self) -> u16;
}

/// Thumb data-processing, format 6
pub trait DpFmt6Bits {
    fn imm8(&self) -> u16;
    fn rd(&self) -> u16;
    fn reg(&self) -> bool;
}

/// Thumb data-processing, format 7
pub trait DpFmt7Bits {
    fn imm7(&self) -> u16;
    fn op6(&self) -> u16;
}

/// Thumb data-processing, format 8
pub trait DpFmt8Bits {
    fn rd(&self) -> u16;
    fn rn(&self) -> u16;
    fn rm(&self) -> u16;
    fn h2(&self) -> bool;
    fn h1(&self) -> bool;
    fn opcd(&self) -> u16;
}


/// Thumb load/store register, format 1
pub trait LsRegFmt1Bits {
    fn rd(&self) -> u16;
    fn rn(&self) -> u16;
    fn imm5(&self) -> u16;
    fn opcd1(&self) -> u16;
}

/// Thumb load/store register, format 2
pub trait LsRegFmt2Bits {
    fn rd(&self) -> u16;
    fn rn(&self) -> u16;
    fn rm(&self) -> u16;
    fn opcd2(&self) -> u16;
}

/// Thumb load/store register, format 3
pub trait LsRegFmt3Bits {
    fn imm8(&self) -> u16;
    fn rd(&self) -> u16;
}

/// Thumb load/store register, format 4
pub trait LsRegFmt4Bits {
    fn imm8(&self) -> u16;
    fn rd(&self) -> u16;
    fn l(&self) -> bool;
}


/// Thumb load/store multiple, format 1
pub trait LsMultiFmt1Bits {
    fn reglist(&self) -> u16;
    fn rn(&self) -> u16;
    fn l(&self) -> bool;
}

/// Thumb load/store multiple, format 2
pub trait LsMultiFmt2Bits {
    fn reglist(&self) -> u16;
    fn r(&self) -> bool;
    fn l(&self) -> bool;
}


/// Thumb exception-generating instructions
pub trait ThumbExcepBits {
    fn imm8(&self) -> u16;
}
pub trait ThumbCondBranchBits {
    fn simm8(&self) -> u16;
    fn cond(&self) -> u16;
}
pub trait ThumbUncondBranchBits {
    fn imm11(&self) -> u16;
    fn h(&self) -> u16;
}

pub trait ThumbBranchExchangeBits {
    fn rm(&self) -> u16;
    fn h2(&self) -> bool;
}
















