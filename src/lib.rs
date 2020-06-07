
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_macros)]

use armbf::*;

pub mod decode;
use crate::decode::*;

/// Decode bits.
const DECODE_BITS_MASK:     u32 = 0b0000_11111111_000000000000_1111_0000;

/// The most general decode bits.
const GROUP_MASK:           u32 = 0b0000_11100000_000000000000_0000_0000;

/// Mask for multiply instructions.
const MULTIPLY_MASK:        u32 = 0b0000_11110000_000000000000_1111_0000;
/// Match for multiply instructions.
const MULTIPLY_MATCH:       u32 = 0b0000_00000000_000000000000_1001_0000;

/// Mask for control instructions.
const CONTROL_MASK:         u32 = 0b0000_11011001_000000000000_0000_0000;
/// Match for control instructions.
const CONTROL_MATCH:        u32 = 0b0000_00010000_000000000000_0000_0000;

/// Mask for load/store miscellaneous instructions.
const LSMISC_MASK:          u32 = 0b0000_11100000_000000000000_1001_0000;
/// Match for load/store miscellaneous instructions.
const LSMISC_MATCH:         u32 = 0b0000_00000000_000000000000_1001_0000;


#[inline(always)]
/// Decode a control instruction.
pub fn arm_decode_control(x: u32) -> ArmInst {
    match get_control_opcd!(x) {
        0b0000 => {
            if bit!(x, 21) { return ArmInst::Mrs_reg; } 
            else { return ArmInst::Msr; }
        }
        0b0001 => {
            if bit!(x, 22) { return ArmInst::Clz; } 
            else { return ArmInst::Bx; }
        },
        0b0010 => ArmInst::Bxj,
        0b0011 => ArmInst::Blx,
        0b0101 => {
            match get_sat_addsub_op!(x) {
                0b00 => ArmInst::Qadd,
                0b01 => ArmInst::Qsub,
                0b10 => ArmInst::QdAdd,
                0b11 => ArmInst::QdSub,
                _ => unreachable!(),
            }
        },
        0b0111 => ArmInst::Bkpt,
        0b1000 | 0b1010 | 0b1100 | 0b1110 => {
            match get_signed_mul_op!(x) {
                0b00 => ArmInst::Smla,
                0b01 => {
                    if bit!(x, 5) { 
                        return ArmInst::Smulw;
                    } else { 
                        return ArmInst::Smlaw;
                    }
                }
                0b10 => ArmInst::Smlal,
                0b11 => ArmInst::Smul,
                _ => unreachable!(),
            }
        },
        _ => unreachable!(),
    }
}

#[inline(always)]
/// Decode a load/store misc. instruction.
pub fn arm_decode_lsmisc(x: u32) -> ArmInst {
    match get_decode_bits_lo!(x) {
        0b1001 => {
            if bit!(x, 22) { return ArmInst::Swpb; } 
            else { return ArmInst::Swp; }
        },
        0b1011 => {
            if bit!(x, 22) { return ArmInst::StrhLdrh_imm; } 
            else { return ArmInst::StrhLdrh_reg; }
        },
        0b1101 | 0b1110 => {
            let tbl = (bit!(x, 20), bit!(x, 22), bit!(x, 5));
            return match tbl {
                (true, false, false) => ArmInst::Ldrsb_reg,
                (true, false, true) =>  ArmInst::Ldrsh_reg,
                (true, true, false) =>  ArmInst::Ldrsb_imm,
                (true, true, true) =>   ArmInst::Ldrsh_imm,
                (false, false, _) =>    ArmInst::StrdLdrd_reg,
                (false, true, _) =>     ArmInst::StrdLdrd_imm,
            }
        },
        _ => unreachable!(),
    }
}

/// Check if this number is a valid control instruction.
macro_rules! is_valid_control_instr { ($val:expr) => {
    (
        (($val & CONTROL_MASK) == CONTROL_MATCH) && 
        !(
            (bit!($val, 25) == false) && 
            (bit!($val, 7) == true) && 
            (bit!($val, 4) == true)
        )
    )
}}

/// Check if this number is a valid load/store misc instruction.
macro_rules! is_valid_lsmisc_instr { ($val:expr) => {
    ($val & LSMISC_MASK) == LSMISC_MATCH
}}

/// Check if this number is a valid multiply instruction.
macro_rules! is_valid_multiply_instr { ($val:expr) => {
    ($val & MULTIPLY_MASK) == MULTIPLY_MATCH
}}


/// Top-level decoder for ARM instructions.
pub fn decode(x: u32) -> ArmInst {

    // Match the top 3 decode bits (breaking the space into 8 groups).
    // NOTE: Most exceptional/specific cases are in the 0b000 group.

    let instr = match get_group!(x) {

        // Data processing (shifter immediate).
        // There are exceptional cases for control, multiply, load/store misc. 
        // instructions.

        0b000 => { 
            if is_valid_lsmisc_instr!(x) { return arm_decode_lsmisc(x); }
            if is_valid_control_instr!(x) { return arm_decode_control(x); }
            if is_valid_multiply_instr!(x) {
                if bit!(x, 23) { return ArmInst::MulMla; } 
                else { return ArmInst::UmulUmla; }
            }

            if bit!(x, 4) { return ArmInst::DpShiftReg; } 
            ArmInst::DpShiftImm
        },

        // Data processing (rotate immediate).
        // There is only one exception for a single control instruction (mrs).

        0b001 => {
            if is_valid_control_instr!(x) { return ArmInst::Mrs_imm; }
            ArmInst::DpRotImm
        },

        // Load/store (immediate offset)
        0b010 => ArmInst::LsImm,

        // Load/store (shifter offset).
        // One exception for media instructions (which we don't support).

        0b011 => { 
            if bit!(x, 4) { return ArmInst::None; }
            ArmInst::LsShift
        },

        // Load/store multiple instructions
        0b100 => ArmInst::LsMulti,

        // Branch instructions
        0b101 => ArmInst::Branch,

        // Coprocessor load/stores
        0b110 => ArmInst::CoprocLs,

        // Coprocessor [register transfer/data processing].
        // One exception for the software interrupt instruction.

        0b111 => { 
            if bit!(x, 24) {
                if get_cond!(x) != 0b1111 { return ArmInst::Swi; }
                return ArmInst::None;
            }

            if bit!(x, 4) { return ArmInst::CoprocRt; }
            ArmInst::CoprocDp
        },
        _ => unreachable!(),
    };
    instr
}


