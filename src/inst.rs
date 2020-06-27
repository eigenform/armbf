//! Defines a set of decode-able/supported ARM and Thumb instructions.

use crate::fields::*;
use armbf_prim::*;

/// The set of supported THUMB instructions.
#[derive(Debug)]
pub enum ThumbInst {
    None,

    RsbImm,

    LdrshReg, 
    LdrbReg, 
    LdrhReg, 
    LdrReg, 
    LdrsbReg,
    StrbReg, 
    StrhReg, 
    StrReg,

    LdrImm2, 
    StrImm2, 
    LdrhImm, 
    StrhImm,

    LdrbImm, 
    StrbImm, 
    LdrImm1, 
    StrImm1,

    SubImm1, 
    AddImm1, 
    SubReg, 
    AddReg1,

    Bx, 
    BlxReg, 
    MovReg, 
    CmpReg2, 
    AddReg2,

    AsrImm, 
    LsrImm, 
    LslImm,

    SubImm2, AddImm2,
    CmpImm, MovImm, 

    AndReg, EorReg, LslReg, LsrReg,
    AsrReg, AdcReg, SbcReg, RorReg,
    TstReg, CmnReg, OrrReg, MulReg,
    BicReg, MvnReg, CmpReg1,

    AddImmSp7, AddImmPc7, SubImmSp7,

    Push, Pop,
    Swi, Bkpt, 
    BranchCond, BranchUncond,

    LdrLit,
    AddImmPc, AddImmSp,
    Stmia, Ldmia,
}

impl ThumbInst {

    #[inline]
    fn decode_dp_fmt5(x: u16) -> ThumbInst {
        match (x & 0b0000_0011_1100_0000) >> 6 {
            0b0000 => ThumbInst::AndReg,
            0b0001 => ThumbInst::EorReg,
            0b0010 => ThumbInst::LslReg,
            0b0011 => ThumbInst::LsrReg,
            0b0100 => ThumbInst::AsrReg,
            0b0101 => ThumbInst::AdcReg,
            0b0110 => ThumbInst::SbcReg,
            0b0111 => ThumbInst::RorReg,
            0b1000 => ThumbInst::TstReg,
            0b1001 => ThumbInst::RsbImm,
            0b1010 => ThumbInst::CmpReg1,
            0b1011 => ThumbInst::CmnReg,
            0b1100 => ThumbInst::OrrReg,
            0b1101 => ThumbInst::MulReg,
            0b1110 => ThumbInst::BicReg,
            0b1111 => ThumbInst::MvnReg,
            _ => panic!("No match {:04x}", x),
        }
    }

    #[inline]
    fn decode_dp_fmt3(x: u16) -> ThumbInst {
        match (x & 0b0001_1000_0000_0000) >> 11 {
            0b00 => ThumbInst::MovImm,
            0b01 => ThumbInst::CmpImm,
            0b10 => ThumbInst::AddImm2,
            0b11 => ThumbInst::SubImm2,
            _ => panic!("No match {:04x}", x),
        }
    }

    #[inline]
    fn decode_lsreg(x: u16) -> ThumbInst {
        match (x & 0b0000_1110_0000_0000) >> 9 {
            0b000 => ThumbInst::StrReg,
            0b001 => ThumbInst::StrhReg,
            0b010 => ThumbInst::StrbReg,
            0b011 => ThumbInst::LdrsbReg,
            0b100 => ThumbInst::LdrReg,
            0b101 => ThumbInst::LdrhReg,
            0b110 => ThumbInst::LdrbReg,
            0b111 => ThumbInst::LdrshReg,
            _ => panic!("No match {:04x}", x),
        }
    }

    #[inline]
    fn decode_dp_special(x: u16) -> ThumbInst {
        match (x & 0b0000_0011_0000_0000) >> 8 {
            // Data-processing Format 8
            0b00 => ThumbInst::AddReg2,
            0b01 => ThumbInst::CmpReg2,
            0b10 => ThumbInst::MovReg,
            // Branch/exchange
            0b11 => {
                if bit!(x, 7) {
                    return ThumbInst::BlxReg;
                } else {
                    return ThumbInst::Bx;
                }
            },
            _ => panic!("No match {:04x}", x),
        }
    }

    #[inline]
    fn decode_dp_fmt1_fmt2(x: u16) -> ThumbInst {
        match (x & 0b0000_0110_0000_0000) >> 9 {
            // Data-processing Format 1
            0b00 => ThumbInst::AddReg1,
            0b01 => ThumbInst::SubReg,

            // Data-processing Format 2
            0b10 => ThumbInst::AddImm1,
            0b11 => ThumbInst::SubImm1,
            _ => panic!("No match {:04x}", x),
        }
    }

    #[inline]
    fn decode_ls_wb_imm(x: u16) -> ThumbInst {
        match (x & 0b0001_1000_0000_0000) >> 11 {
            0b00 => ThumbInst::StrImm1,
            0b01 => ThumbInst::LdrImm1,
            0b10 => ThumbInst::StrbImm,
            0b11 => ThumbInst::LdrbImm,
            _ => panic!("No match {:04x}", x),
        }
    }

    #[inline]
    fn decode_ls_half_stack(x: u16) -> ThumbInst {
        match (x & 0b0001_1000_0000_0000) >> 11 {
            // Load/store halfword
            0b00 => ThumbInst::StrhImm,
            0b01 => ThumbInst::LdrhImm,
            // Load/store stack
            0b10 => ThumbInst::StrImm2,
            0b11 => ThumbInst::LdrImm2,
            _ => panic!("No match {:04x}", x),
        }
    }

    #[inline]
    fn decode_misc(x: u16) -> ThumbInst {
        match (x & 0b0000_1111_0000_0000) >> 8 {
            0b0000 => {
                if bit!(x, 7) {
                    return ThumbInst::SubImmSp7;
                } else {
                    return ThumbInst::AddImmSp7;
                }
            },
            0b0100 |
            0b0101 => return ThumbInst::Push,

            0b1100 |
            0b1101 => return ThumbInst::Pop,

            0b1110 => return ThumbInst::Bkpt,
            _ => return ThumbInst::None,
        }
    }

    #[inline]
    fn decode_cond_branch(x: u16) -> ThumbInst {
        match (x & 0b0000_1111_0000_0000) >> 8 {
            0b1110 => return ThumbInst::None,
            0b1111 => return ThumbInst::Swi,
            _ => return ThumbInst::BranchCond,
        }
    }

}


impl ThumbInst {
    /// Decode a THUMB instruction.
    pub fn decode(x: u16) -> ThumbInst {
        match (x & 0b1110_0000_0000_0000) >> 13 {
            0b000 => {
                match (x & 0b0001_1000_0000_0000) >> 11 {
                    // Data-processing Format 4
                    0b00 => ThumbInst::LslImm,
                    0b01 => ThumbInst::LsrImm,
                    0b10 => ThumbInst::AsrImm,
                    // Data-processing format 1/2
                    0b11 => ThumbInst::decode_dp_fmt1_fmt2(x),
                    _ => panic!("No match {:04x}", x),
                }
            },
            0b001 => ThumbInst::decode_dp_fmt3(x),
            0b010 => {
                match (x & 0b0001_1100_0000_0000) >> 10 {
                    0b000 => ThumbInst::decode_dp_fmt5(x),
                    0b001 => ThumbInst::decode_dp_special(x),
                    0b010 |
                    0b011 => ThumbInst::LdrLit,

                    0b100 |
                    0b101 |
                    0b110 |
                    0b111 => ThumbInst::decode_lsreg(x), 
                    _ => panic!("No match {:04x}", x),
                }
            },
            0b011 => ThumbInst::decode_ls_wb_imm(x),
            0b100 => ThumbInst::decode_ls_half_stack(x),
            0b101 => {
                match (x & 0b0001_1000_0000_0000) >> 11 {
                    0b00 => ThumbInst::AddImmPc,
                    0b01 => ThumbInst::AddImmSp,
                    0b10 |
                    0b11 => ThumbInst::decode_misc(x),
                    _ => panic!("No match {:04x}", x),
                }
            }
            0b110 => {
                match (x & 0b0001_1000_0000_0000) >> 11 {
                    0b00 => ThumbInst::Stmia,
                    0b01 => ThumbInst::Ldmia,
                    0b10 |
                    0b11 => ThumbInst::decode_cond_branch(x),
                    _ => panic!("No match {:04x}", x),
                }
            },
            // TODO
            0b111 => return ThumbInst::None,
            _ => panic!("No match {:04x}", x),
        }
    }
}


/// The set of supported ARMv5 instructions.
#[derive(Debug)]
pub enum ArmInst {
    None,

    // Control (status register)
    MsrReg, MsrImm, Mrs,

    // Control (misc.)
    Clz, Bx, BlxReg,

    // Control (saturated add/sub)
    Qadd, Qsub, QdAdd, QdSub,

    // Control (extended multiplies)
    SmlaXy, SmulwY, SmlawY, SmlalXy, SmulXy,

    // Load/store halfword
    StrhImm, LdrhImm, StrhReg, LdrhReg,

    // Load/store misc.
    LdrsbReg, LdrshReg, LdrsbImm, LdrshImm,
    StrdReg, LdrdReg, StrdImm, LdrdImm,

    // Multiplies
    Mul, Mla, Umull, Umlal, Smull, Smlal,

    // Load/store multiple
    Stmia, Stmib, Stmda, Stmdb, Ldmia, Ldmib, Ldmda, Ldmdb,

    // Load/store 
    StrReg, LdrReg, StrbReg, LdrbReg,
    StrImm, LdrImm, StrbImm, LdrbImm,

    // Branching
    B, Bl, BlxImm,

    // Misc.
    Bkpt, Swi, Swp, Swpb,

    // Coprocessor (register transfer)
    Mrc, Mcr,

    // Coprocessor
    CoprocLs, CoprocDp,

    // Data processing (rotate immediate)
    AndRotImm, EorRotImm, SubRotImm, RsbRotImm,
    AddRotImm, AdcRotImm, SbcRotImm, RscRotImm,
    TstRotImm, TeqRotImm, CmpRotImm, CmnRotImm,
    OrrRotImm, MovRotImm, BicRotImm, MvnRotImm,

    // Data processing (shift immediate)
    AndShiftImm, EorShiftImm, SubShiftImm, RsbShiftImm,
    AddShiftImm, AdcShiftImm, SbcShiftImm, RscShiftImm,
    TstShiftImm, TeqShiftImm, CmpShiftImm, CmnShiftImm,
    OrrShiftImm, MovShiftImm, BicShiftImm, MvnShiftImm,

    // Data processing (shift register)
    AndShiftReg, EorShiftReg, SubShiftReg, RsbShiftReg,
    AddShiftReg, AdcShiftReg, SbcShiftReg, RscShiftReg,
    TstShiftReg, TeqShiftReg, CmpShiftReg, CmnShiftReg,
    OrrShiftReg, MovShiftReg, BicShiftReg, MvnShiftReg,

}

/// Decoding for various sub-groups of instructions.
impl ArmInst {

    /// Decode a control instruction.
    #[inline(always)]
    fn decode_ctrl(x: u32) -> ArmInst {
        match get_control_opcd!(x) {
            0b0000 => match bit!(x, 21) {
                true => ArmInst::MsrReg,
                false => ArmInst::Mrs,
            },
            0b0001 => match bit!(x, 22) { 
                true => ArmInst::Clz,
                false => ArmInst::Bx, 
            },
            0b0011 => ArmInst::BlxReg,
            0b0101 => match get_sat_addsub_op!(x) {
                0b00 => ArmInst::Qadd,
                0b01 => ArmInst::Qsub,
                0b10 => ArmInst::QdAdd,
                0b11 => ArmInst::QdSub,
                _ => unreachable!(),
            },
            0b0111 => ArmInst::Bkpt,
            0b1000 | 
            0b1010 | 
            0b1100 | 
            0b1110 => match get_signed_mul_op!(x) {
                0b00 => ArmInst::SmlaXy,
                0b01 => match bit!(x, 5) {
                    true => ArmInst::SmulwY,
                    false => ArmInst::SmlawY,
                },
                0b10 => ArmInst::SmlalXy,
                0b11 => ArmInst::SmulXy,
                _ => unreachable!(),
            },
            _ => ArmInst::None,
        }
    }

    /// Decode a load/store misc. instruction.
    #[inline(always)]
    fn decode_lsmisc(x: u32) -> ArmInst {
        match get_decode_bits_lo!(x) {
            0b1001 => match get_b!(x) {
                false => ArmInst::Swp,
                true => ArmInst::Swpb,
            }
            0b1011 => match (bit!(x, 22), get_l!(x)) {
                (true, false) => ArmInst::StrhImm,
                (true, true) => ArmInst::LdrhImm,
                (false, false) => ArmInst::StrhReg,
                (false, true) => ArmInst::LdrhReg,
            },
            0b1101 | 
            0b1110 | 
            0b1111 => match (bit!(x, 20), bit!(x, 22), bit!(x, 5)) {
                (true, false, false) => ArmInst::LdrsbReg,
                (true, false, true) => ArmInst::LdrshReg,
                (true, true, false) => ArmInst::LdrsbImm,
                (true, true, true) => ArmInst::LdrshImm,

                (false, false, false) => ArmInst::LdrdReg,
                (false, false, true) => ArmInst::StrdReg,
                (false, true, false) => ArmInst::LdrdImm,
                (false, true, true) => ArmInst::StrdImm,
            },
            _ => unreachable!("Instruction {:08x} {:032b}", x, x),
        }
    }

    /// Decode a multiply instruction.
    #[inline(always)]
    fn decode_mul(x: u32) -> ArmInst {
        match (bit!(x, 23), bit!(x, 22), bit!(x, 21)) {
            (false, false, false) => return ArmInst::Mul,
            (false, false, true) => return ArmInst::Mla,

            (true, false, false) => return ArmInst::Umull,
            (true, false, true) => return ArmInst::Umlal,

            (true, true, false) => return ArmInst::Smull,
            (true, true, true) => return ArmInst::Smlal,
            _ => ArmInst::None,
        }
    }
}

impl ArmInst {

    /// Top-level decoder for ARM instructions.
    ///
    /// Given some 32-bit number, return the corresponding ARM instruction.
    pub fn decode(x: u32) -> Self { 
        match get_group!(x) {
            0b000 => { 
                if is_valid_multiply_instr!(x) { 
                    return ArmInst::decode_mul(x); 
                }
                if is_valid_lsmisc_instr!(x) { 
                    return ArmInst::decode_lsmisc(x); 
                }
                if is_valid_control_instr!(x) { 
                    return ArmInst::decode_ctrl(x); 
                }
                if bit!(x, 4) { 
                    return match Opcode::from_u32(get_opcd!(x)) {
                        Opcode::And => ArmInst::AndShiftReg,
                        Opcode::Eor => ArmInst::EorShiftReg,
                        Opcode::Sub => ArmInst::SubShiftReg,
                        Opcode::Rsb => ArmInst::RsbShiftReg,
                        Opcode::Add => ArmInst::AddShiftReg,
                        Opcode::Adc => ArmInst::AdcShiftReg,
                        Opcode::Sbc => ArmInst::SbcShiftReg,
                        Opcode::Rsc => ArmInst::RscShiftReg,
                        Opcode::Tst => ArmInst::TstShiftReg,
                        Opcode::Teq => ArmInst::TeqShiftReg,
                        Opcode::Cmp => ArmInst::CmpShiftReg,
                        Opcode::Cmn => ArmInst::CmnShiftReg,
                        Opcode::Orr => ArmInst::OrrShiftReg,
                        Opcode::Mov => ArmInst::MovShiftReg,
                        Opcode::Bic => ArmInst::BicShiftReg,
                        Opcode::Mvn => ArmInst::MvnShiftReg,
                        _ => panic!(),
                    }
                }

                match Opcode::from_u32(get_opcd!(x)) {
                    Opcode::And => ArmInst::AndShiftImm,
                    Opcode::Eor => ArmInst::EorShiftImm,
                    Opcode::Sub => ArmInst::SubShiftImm,
                    Opcode::Rsb => ArmInst::RsbShiftImm,
                    Opcode::Add => ArmInst::AddShiftImm,
                    Opcode::Adc => ArmInst::AdcShiftImm,
                    Opcode::Sbc => ArmInst::SbcShiftImm,
                    Opcode::Rsc => ArmInst::RscShiftImm,
                    Opcode::Tst => ArmInst::TstShiftImm,
                    Opcode::Teq => ArmInst::TeqShiftImm,
                    Opcode::Cmp => ArmInst::CmpShiftImm,
                    Opcode::Cmn => ArmInst::CmnShiftImm,
                    Opcode::Orr => ArmInst::OrrShiftImm,
                    Opcode::Mov => ArmInst::MovShiftImm,
                    Opcode::Bic => ArmInst::BicShiftImm,
                    Opcode::Mvn => ArmInst::MvnShiftImm,
                    _ => panic!(),
                }
            },
            0b001 => {
                if is_valid_control_instr!(x) { 
                    return ArmInst::MsrImm; 
                }
                match Opcode::from_u32(get_opcd!(x)) {
                    Opcode::And => ArmInst::AndRotImm,
                    Opcode::Eor => ArmInst::EorRotImm,
                    Opcode::Sub => ArmInst::SubRotImm,
                    Opcode::Rsb => ArmInst::RsbRotImm,
                    Opcode::Add => ArmInst::AddRotImm,
                    Opcode::Adc => ArmInst::AdcRotImm,
                    Opcode::Sbc => ArmInst::SbcRotImm,
                    Opcode::Rsc => ArmInst::RscRotImm,
                    Opcode::Tst => ArmInst::TstRotImm,
                    Opcode::Teq => ArmInst::TeqRotImm,
                    Opcode::Cmp => ArmInst::CmpRotImm,
                    Opcode::Cmn => ArmInst::CmnRotImm,
                    Opcode::Orr => ArmInst::OrrRotImm,
                    Opcode::Mov => ArmInst::MovRotImm,
                    Opcode::Bic => ArmInst::BicRotImm,
                    Opcode::Mvn => ArmInst::MvnRotImm,
                    _ => panic!(),
                }
            },
            0b010 => match (get_l!(x), get_b!(x)) {
                (false, false) => ArmInst::StrImm,
                (false, true) => ArmInst::StrbImm,
                (true, false) => ArmInst::LdrImm,
                (true, true) => ArmInst::LdrbImm,
            },
            0b011 => { 
                if bit!(x, 4) { return ArmInst::None; }
                match (get_l!(x), get_b!(x)) {
                    (false, false) => ArmInst::StrReg,
                    (false, true) => ArmInst::StrbReg,
                    (true, false) => ArmInst::LdrReg,
                    (true, true) => ArmInst::LdrbReg,
                }
            },
            0b100 => match (get_l!(x), get_p!(x), get_u!(x)) {
                (false, false, false) => ArmInst::Stmda,
                (false, false, true) => ArmInst::Stmia,
                (false, true, false) => ArmInst::Stmdb,
                (false, true, true) => ArmInst::Stmib,

                (true, false, false) => ArmInst::Ldmda,
                (true, false, true) => ArmInst::Ldmia,
                (true, true, false) => ArmInst::Ldmdb,
                (true, true, true) => ArmInst::Ldmib,
            },
            0b101 => {
                if get_cond!(x) == 0b1111 { return ArmInst::BlxImm; } 
                match get_link!(x) {
                    true => ArmInst::Bl,
                    false => ArmInst::B,
                }
            },
            0b110 => ArmInst::CoprocLs,
            0b111 => { 
                if bit!(x, 24) {
                    if get_cond!(x) != 0b1111 { return ArmInst::Swi; }
                    return ArmInst::None;
                }
                if bit!(x, 4) { 
                    if bit!(x, 20) { return ArmInst::Mrc; } 
                    else { 
                        return ArmInst::Mcr; 
                    }
                }
                ArmInst::CoprocDp
            },
            _ => unreachable!(),
        }
    }
}


#[cfg(test)]
mod test {
    use crate::inst::*;
    #[test]
    fn thumb_decode() {
        for i in 0..0x800u16 {
            println!("{:04x} {:?}", i << 5, ThumbInst::decode(i << 5));
        }
    }
}
