//! Wrapper types (over primitives) for representing instructions.
//!
//! The following structures are newtypes which represent different types of 
//! instructions. Each type derives a set of traits that allows access to some
//! set of bitfields relevant to pulling some useful information out of the
//! instruction.

use armbf_derive::*;

use crate::traits::*;

/// Make declaring ARM newtypes somewhat easier to look at.
///
/// NOTE: This macro implicitly derives some traits representing bitfields 
/// that are common to all types of instructions.
macro_rules! declare_instr_fields { 
    ($name:ident, $($trait:ident),*) => {
        #[repr(transparent)]
        #[derive(Debug, InstBits, $($trait),*)]
        pub struct $name(pub u32);
}}

declare_instr_fields!(DpRotImmBf,       DpBits, RegBits, RotBits, ImmBits);
declare_instr_fields!(DpShiftBf,        DpBits, RegBits, ShiftBits);
declare_instr_fields!(MulBf,            MultiplyBits, RegBits);
declare_instr_fields!(SatBf,            RegBits);
declare_instr_fields!(ClzBf,            RegBits);
declare_instr_fields!(LsImmBf,          LsBits, RegBits, ImmBits);
declare_instr_fields!(LsShiftBf,        LsBits, RegBits, ShiftBits);
declare_instr_fields!(LsMultiBf,        LsBits, RegBits, LsMultiBits);
declare_instr_fields!(LsMiscBf,         LsBits, RegBits, ImmBits);
declare_instr_fields!(SwpBf,            LsBits, RegBits);
declare_instr_fields!(BranchBf,         BranchBits, ImmBits, RegBits);
declare_instr_fields!(BxBf,             RegBits);
declare_instr_fields!(CoprocBf,         CoprocBits, LsBits, ImmBits, RegBits);
declare_instr_fields!(StatusBf,         SrBits, RegBits, ImmBits, RotBits);
declare_instr_fields!(SwiBf,            ImmBits);
declare_instr_fields!(BkptBf,           ImmBits);



/// Make declaring Thumb newtypes somewhat easier to look at.
macro_rules! declare_thumb_fields { 
    ($name:ident, $($trait:ident),*) => {
        #[repr(transparent)]
        #[derive(Debug, $($trait),*)]
        pub struct $name(pub u16);
}}


declare_thumb_fields!(ThumbLsMultiBf,   LsMultiFmt1Bits);
declare_thumb_fields!(PushPopBf,        LsMultiFmt2Bits);
declare_thumb_fields!(ThumbExcepBf,     ThumbExcepBits);


