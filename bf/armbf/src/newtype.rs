//! Wrapper types (over primitives) for representing instructions.
//!
//! The following structures are newtypes which represent different types of 
//! instructions. Each type derives a set of traits that allows access to some
//! set of bitfields relevant to pulling some useful information out of the
//! instruction.

use armbf_derive::*;
use armbf_prim::*;

use crate::traits::*;

/// Make declaring these newtypes somewhat easier to look at.
///
/// NOTE: This macro implicitly derives some traits representing bitfields 
/// that are common to all types of instructions.
macro_rules! declare_instr_fields { 
    ($name:ident, $($trait:ident),*) => {
        #[repr(transparent)]
        #[derive(Debug, InstBits, $($trait),*)]
        pub struct $name(pub u32);
}}

declare_instr_fields!(DpShiftImmBf, DpBits, RegBits, ShiftBits);
declare_instr_fields!(DpShiftRegBf, DpBits, RegBits, ShiftBits);
declare_instr_fields!(DpRotImmBf, DpBits, RegBits, RotBits, ImmBits);

pub struct DpMultiply(pub u32);

declare_instr_fields!(LsImm, LsBits, RegBits, ImmBits);
declare_instr_fields!(LsShift, LsBits, RegBits, ShiftBits);
declare_instr_fields!(LsMulti, LsBits, RegBits, LsMultiBits);
declare_instr_fields!(Branch, BranchBits, ImmBits);

#[repr(transparent)]
pub struct CoprocLs(pub u32);
#[repr(transparent)]
pub struct CoprocDp(pub u32);
#[repr(transparent)]
pub struct CoprocRt(pub u32);

declare_instr_fields!(Swi, ImmBits);

