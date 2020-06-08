//! Exposes functions for decoding ARM instructions.

#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
//#![allow(unused_imports)]

use armbf::newtype::*;
use armbf::inst::*;
use armbf_prim::*;

/// Decode a control instruction.
#[inline(always)]
pub fn arm_decode_control(x: u32) -> ArmInst {
    match get_control_opcd!(x) {
        0b0000 => {
            if bit!(x, 21) { return ArmInst::MrsReg; } 
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

/// Decode a load/store misc. instruction.
#[inline(always)]
pub fn arm_decode_lsmisc(x: u32) -> ArmInst {
    match get_decode_bits_lo!(x) {
        0b1001 => {
            if bit!(x, 22) { return ArmInst::Swpb; } 
            else { return ArmInst::Swp; }
        },
        0b1011 => {
            if bit!(x, 22) { return ArmInst::StrhLdrhImm; } 
            else { return ArmInst::StrhLdrhReg; }
        },
        0b1101 | 0b1110 => {
            let tbl = (bit!(x, 20), bit!(x, 22), bit!(x, 5));
            return match tbl {
                (true, false, false) => ArmInst::LdrsbReg,
                (true, false, true) =>  ArmInst::LdrshReg,
                (true, true, false) =>  ArmInst::LdrsbImm,
                (true, true, true) =>   ArmInst::LdrshImm,
                (false, false, _) =>    ArmInst::StrdLdrdReg,
                (false, true, _) =>     ArmInst::StrdLdrdImm,
            }
        },
        _ => unreachable!(),
    }
}


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

            if bit!(x, 4) { return ArmInst::DpShiftReg(DpShiftRegBf(x)); } 
            ArmInst::DpShiftImm(DpShiftImmBf(x))
        },

        // Data processing (rotate immediate).
        // There is only one exception for a single control instruction (mrs).

        0b001 => {
            if is_valid_control_instr!(x) { return ArmInst::MrsImm; }
            ArmInst::DpRotImm(DpRotImmBf(x))
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


