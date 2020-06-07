//! Wrapper types around unsigned 32-bit numeric representations of varous
//! types of ARM instructions.
//!
//! Each of the traits defined in this crate have a corresponding derive macro 
//! defined within the armbf_derive crate. The derive macros depend on all of
//! the generic bitfield getter/setter macros defined in the prim module.

#![allow(unused_macros)]
#![allow(unused_attributes)]


#[macro_use]
use armbf_derive::*;

#[macro_use]
pub mod prim;


/* These traits describe some set of bitfields that may or may not belong to a 
 * particular group/class/type of ARM instructions. All of the functions in 
 * these traits have some corresponding macro in the prim module which perform 
 * some operation on a u32.
 */

/// Accessors common to all instructions.
pub trait InstCommon {
    fn cond(&self) -> u32;
    fn group(&self) -> u32;
}

/// Accessors common to data processing instructions.
pub trait DpCommon {
    fn opcd(&self) -> u32;
    fn s(&self) -> bool;
}

/// Accessors common to load/store multiple instructions.
pub trait LsMultiCommon {
    fn s(&self) -> bool;
    fn reglist(&self) -> u32;
}

/// Accessors common to load/store instructions.
pub trait LsCommon {
    fn p(&self) -> bool;
    fn u(&self) -> bool;
    fn b(&self) -> bool;
    fn w(&self) -> bool;
    fn l(&self) -> bool;
}

/// Accessors for immediates.
pub trait ImmCommon {
    fn imm8(&self) -> u32;
    fn imm12(&self) -> u32;
    fn imm24(&self) -> u32;
}

/// Accessors for instructions that modify the status registers.
pub trait SrCommon {
    fn field_mask(&self) -> u32;
}

/// Accessors in branching instructions.
pub trait BranchCommon { fn link(&self) -> bool; }

/// Accessors for rotate instructions.
pub trait RotCommon { fn rot_imm(&self) -> u32; }

/// Accessors for shifter instructions.
pub trait ShiftCommon {
    fn shift_imm(&self) -> u32;
    fn shift(&self) -> u32;
}

/// Accessors common to coprocessor instructions.
pub trait CoprocCommon {
    fn opcd1(&self) -> u32;
    fn opcd1_rt(&self) -> u32;
    fn cp_num(&self) -> u32;
    fn opcd2(&self) -> u32;
    fn crn(&self) -> u32;
    fn crd(&self) -> u32;
    fn crm(&self) -> u32;
}

/// Accessors for common register fields.
pub trait RegCommon {
    fn rn(&self) -> u32;
    fn rd(&self) -> u32;
    fn rm(&self) -> u32;
    fn rs(&self) -> u32;
}


/* The following structures are wrapper types around u32 which represent 
 * different types/groups of instructions.  Each type derives a set of traits 
 * that allow us to access to some bitfields.
 */

#[derive(InstCommon, DpCommon, RegCommon, ShiftCommon, SrCommon)]
pub struct DpShiftImm(pub u32);

#[derive(InstCommon, DpCommon, RegCommon, ShiftCommon)]
pub struct DpShiftReg(pub u32);

#[derive(InstCommon, DpCommon, RegCommon, RotCommon, ImmCommon, SrCommon)]
pub struct DpRotImm(pub u32);

pub struct DpMultiply(pub u32);

#[derive(InstCommon, LsCommon, RegCommon, ImmCommon)]
pub struct LsImm(pub u32);

#[derive(InstCommon, LsCommon, RegCommon, ShiftCommon)]
pub struct LsShift(pub u32);

#[derive(InstCommon, LsCommon, RegCommon, LsMultiCommon)]
pub struct LsMulti(pub u32);

#[derive(InstCommon, BranchCommon, ImmCommon)]
pub struct Branch(pub u32);

pub struct CoprocLs(pub u32);
pub struct CoprocDp(pub u32);
pub struct CoprocRt(pub u32);

#[derive(InstCommon, ImmCommon)]
pub struct Swi(pub u32);



#[cfg(test)]
mod tests {
    #[test]
    fn prim_macro_test() {
        let val = 0x8001_0001u32;
        assert_eq!(bit!(val, 0), true);
        assert_eq!(bit!(val, 16), true);
        assert_eq!(bit!(val, 31), true);
    }
}

