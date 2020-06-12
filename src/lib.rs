//! Exposes functions for decoding ARM instructions.

#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
//#![allow(unused_imports)]

pub mod disas;

use armbf::newtype::*;
use armbf::inst::*;
use armbf_prim::*;

/// Decode a control instruction.
#[inline(always)]
pub fn arm_decode_control(x: u32) -> ArmInst {
    match get_control_opcd!(x) {
        0b0000 => {
            if bit!(x, 21) { 
                return ArmInst::MsrReg(StatusBf(x)); 
            } 
            else { 
                return ArmInst::Mrs(StatusBf(x)); 
            }
        }
        0b0001 => {
            if bit!(x, 22) { 
                return ArmInst::Clz(ClzBf(x)); 
            } else { 
                return ArmInst::Bx(BxBf(x)); 
            }
        },
        0b0010 => ArmInst::Bxj,
        0b0011 => ArmInst::BlxReg(BranchBf(x)),
        0b0101 => {
            match get_sat_addsub_op!(x) {
                0b00 => ArmInst::Qadd(SatBf(x)),
                0b01 => ArmInst::Qsub(SatBf(x)),
                0b10 => ArmInst::QdAdd(SatBf(x)),
                0b11 => ArmInst::QdSub(SatBf(x)),
                _ => unreachable!(),
            }
        },
        0b0111 => ArmInst::Bkpt(BkptBf(x)),
        0b1000 | 0b1010 | 0b1100 | 0b1110 => {
            match get_signed_mul_op!(x) {
                0b00 => ArmInst::SmlaXy(MulBf(x)),
                0b01 => {
                    if bit!(x, 5) { 
                        return ArmInst::SmulwY(MulBf(x));
                    } else { 
                        return ArmInst::SmlawY(MulBf(x));
                    }
                }
                0b10 => ArmInst::SmlalXy(MulBf(x)),
                0b11 => ArmInst::SmulXy(MulBf(x)),
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
        0b1001 => ArmInst::Swp(SwpBf(x)),
        0b1011 => {
            if bit!(x, 22) { return ArmInst::LsHalfImm(LsHalfImmBf(x)); } 
            else { return ArmInst::LsHalfReg(LsHalfRegBf(x)); }
        },
        0b1101 | 0b1110 | 0b1111 => {
            let tbl = (bit!(x, 20), bit!(x, 22), bit!(x, 5));
            return match tbl {
                (true, false, false) => ArmInst::LdrsbReg(LdrsbRegBf(x)),
                (true, false, true) => ArmInst::LdrshReg(LdrshRegBf(x)),
                (true, true, false) => ArmInst::LdrsbImm(LdrsbImmBf(x)),
                (true, true, true) => ArmInst::LdrshImm(LdrshImmBf(x)),
                (false, false, _) => ArmInst::LsDoubleReg(LsDoubleRegBf(x)),
                (false, true, _) => ArmInst::LsDoubleImm(LsDoubleImmBf(x)),
            }
        },
        _ => unreachable!("Instruction {:08x} {:032b}", x, x),
    }
}

#[inline(always)]
pub fn arm_decode_multiply(x: u32) -> ArmInst {
    match (bit!(x, 23), bit!(x, 22), bit!(x, 21)) {
        (false, false, false) => return ArmInst::Mul(MulBf(x)),
        (false, false, true) => return ArmInst::Mla(MulBf(x)),

        (true, false, false) => return ArmInst::Umull(MulBf(x)), 
        (true, false, true) => return ArmInst::Umlal(MulBf(x)), 

        (true, true, false) => return ArmInst::Smull(MulBf(x)),
        (true, true, true) => return ArmInst::Smlal(MulBf(x)),
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
            if is_valid_multiply_instr!(x) { return arm_decode_multiply(x); }
            if is_valid_lsmisc_instr!(x) { return arm_decode_lsmisc(x); }
            if is_valid_control_instr!(x) { return arm_decode_control(x); }

            if bit!(x, 4) { return ArmInst::DpShiftReg(DpShiftRegBf(x)); } 
            ArmInst::DpShiftImm(DpShiftImmBf(x))
        },

        // Data processing (rotate immediate).
        // There is only one exception for a control instruction (msr).

        0b001 => {
            if is_valid_control_instr!(x) { return ArmInst::MsrImm(StatusBf(x)); }
            ArmInst::DpRotImm(DpRotImmBf(x))
        },

        // Load/store (immediate offset)
        0b010 => ArmInst::LsImm(LsImmBf(x)),

        // Load/store (shifter offset).
        // One exception for media instructions (which we don't support).

        0b011 => { 
            if bit!(x, 4) { return ArmInst::None; }
            ArmInst::LsShift(LsShiftBf(x))
        },

        // Load/store multiple instructions
        0b100 => ArmInst::LsMulti(LsMultiBf(x)),

        // Branch instructions
        0b101 => {
            if get_cond!(x) == 0b1111 {
                return ArmInst::BlxImm(BranchBf(x));
            } else {
                return ArmInst::Branch(BranchBf(x));
            }
        },

        // Coprocessor load/stores
        0b110 => ArmInst::CoprocLs(CoprocBf(x)),

        // Coprocessor [register transfer/data processing].
        // One exception for the software interrupt instruction.

        0b111 => { 
            if bit!(x, 24) {
                if get_cond!(x) != 0b1111 { return ArmInst::Swi(SwiBf(x)); }
                return ArmInst::None;
            }

            if bit!(x, 4) { 
                if bit!(x, 20) { 
                    return ArmInst::Mrc(CoprocBf(x));
                } else {
                    return ArmInst::Mcr(CoprocBf(x));
                }
            }

            ArmInst::CoprocDp(CoprocBf(x))
        },
        _ => unreachable!(),
    };
    instr
}


