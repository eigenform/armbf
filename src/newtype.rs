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


/// Bitfield for data-processing (rotate immediate) instructions.
declare_instr_fields!(DpRotImmBf,       DpBits, RegBits, RotBits, ImmBits);

/// Bitfield for data-processing (shift immediate/register) instructions.
declare_instr_fields!(DpShiftBf,        DpBits, RegBits, ShiftBits);

/// Bitfield for multiply instructions.
declare_instr_fields!(MulBf,            MultiplyBits, RegBits);

/// Bitfield for saturating add/sub instructions.
declare_instr_fields!(SatBf,            RegBits);

/// Bitfield for the `clz` instruction.
declare_instr_fields!(ClzBf,            RegBits);

/// Bitfield for load/store immediate instructions.
declare_instr_fields!(LsImmBf,          LsBits, RegBits, ImmBits);

/// Bitfield for load/store shift instructions.
declare_instr_fields!(LsShiftBf,        LsBits, RegBits, ShiftBits);

/// Bitfield for load/store multiple instructions.
declare_instr_fields!(LsMultiBf,        LsBits, RegBits, LsMultiBits);

/// Bitfield for load/store miscellaneous instructions.
declare_instr_fields!(LsMiscBf,         LsBits, RegBits, ImmBits);

/// Bitfield for the `swp` and `swpb` instructions.
declare_instr_fields!(SwpBf,            LsBits, RegBits);

/// Bitfield for branching instructions.
declare_instr_fields!(BranchBf,         BranchBits, ImmBits, RegBits);

/// Bitfield for the `bx` instruction.
declare_instr_fields!(BxBf,             RegBits);

/// Bitfield for coprocessor instructions.
declare_instr_fields!(CoprocBf,         CoprocBits, LsBits, ImmBits, RegBits);

/// Bitfield for status register instructions.
declare_instr_fields!(StatusBf,         SrBits, RegBits, ImmBits, RotBits);

/// Bitfield for the `swi` (or the `svc`) instruction.
declare_instr_fields!(SwiBf,            ImmBits);

/// Bitfield for the `bkpt` instruction.
declare_instr_fields!(BkptBf,           ImmBits);

