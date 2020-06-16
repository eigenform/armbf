//! Defines a set of decode-able ARM instructions.

use crate::newtype::*;
use crate::fields::*;
use armbf_prim::*;

use std::marker::Copy;

/// An ARMv5 lookup table.
pub struct ArmLut<T: ArmLutEntry> { pub data: [T; 0x1000] }

/// Implemented on all types store-able by some ArmLut.
pub trait ArmLutEntry { 
    /// A map from ArmInst to some ArmLutEntry.
    fn from_inst(inst: ArmInst) -> Self;
}

/// Creates a new ArmLookupTable for some T.
pub fn CreateArmLut<T: ArmLutEntry + Copy>(default_entry: T) -> ArmLut<T> {
    let mut lut = ArmLut {
        data: [default_entry; 0x1000],
    };

    // The details of how to obtain an entry [of type T] are left to the user.
    for i in 0..0x1000 {
        let inst: u32 = ((i & 0x0ff0) << 16) | ((i & 0x000f) << 4);
        lut.data[i as usize] = T::from_inst(ArmInst::from_u32(inst));
    }
    lut
}




#[derive(Debug)]
pub enum ArmInst {
    None,

    MsrReg,
    MsrImm,
    Mrs,

    Clz,
    Bx,
    BlxReg,

    Qadd,
    Qsub,
    QdAdd,
    QdSub,

    SmlaXy,
    SmulwY,
    SmlawY,
    SmlalXy,
    SmulXy,

    Swp,
    Swpb,

    StrhImm,
    LdrhImm,
    StrhReg,
    LdrhReg,

    LdrsbReg,
    LdrshReg,
    LdrsbImm,
    LdrshImm,

    StrdReg,
    LdrdReg,
    StrdImm,
    LdrdImm,

    Mul,
    Mla,
    Umull,
    Umlal,
    Smull,
    Smlal,

    Stmia,
    Stmib,
    Stmda,
    Stmdb,
    Ldmia,
    Ldmib,
    Ldmda,
    Ldmdb,

    StrReg,
    LdrReg,
    StrbReg,
    LdrbReg,

    StrImm,
    LdrImm,
    StrbImm,
    LdrbImm,

    Bl,
    B,
    BlxImm,

    Bkpt,
    Swi,

    Mrc,
    Mcr,

    CoprocLs,
    CoprocDp,

    AndRotImm,
    EorRotImm,
    SubRotImm,
    RsbRotImm,
    AddRotImm,
    AdcRotImm,
    SbcRotImm,
    RscRotImm,
    TstRotImm,
    TeqRotImm,
    CmpRotImm,
    CmnRotImm,
    OrrRotImm,
    MovRotImm,
    BicRotImm,
    MvnRotImm,

    AndShiftImm,
    EorShiftImm,
    SubShiftImm,
    RsbShiftImm,
    AddShiftImm,
    AdcShiftImm,
    SbcShiftImm,
    RscShiftImm,
    TstShiftImm,
    TeqShiftImm,
    CmpShiftImm,
    CmnShiftImm,
    OrrShiftImm,
    MovShiftImm,
    BicShiftImm,
    MvnShiftImm,

    AndShiftReg,
    EorShiftReg,
    SubShiftReg,
    RsbShiftReg,
    AddShiftReg,
    AdcShiftReg,
    SbcShiftReg,
    RscShiftReg,
    TstShiftReg,
    TeqShiftReg,
    CmpShiftReg,
    CmnShiftReg,
    OrrShiftReg,
    MovShiftReg,
    BicShiftReg,
    MvnShiftReg,

}

impl ArmInst {

    /// Decode a control instruction.
    #[inline(always)]
    fn from_u32_ctrl(x: u32) -> ArmInst {
        match get_control_opcd!(x) {
            0b0000 => {
                if bit!(x, 21) { 
                    return ArmInst::MsrReg;
                } 
                else { 
                    return ArmInst::Mrs;
                }
            }
            0b0001 => {
                if bit!(x, 22) { 
                    return ArmInst::Clz;
                } else { 
                    return ArmInst::Bx;
                }
            },
            0b0011 => ArmInst::BlxReg,
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
                    0b00 => ArmInst::SmlaXy,
                    0b01 => {
                        if bit!(x, 5) { 
                            return ArmInst::SmulwY;
                        } else { 
                            return ArmInst::SmlawY;
                        }
                    }
                    0b10 => ArmInst::SmlalXy,
                    0b11 => ArmInst::SmulXy,
                    _ => unreachable!(),
                }
            },
            //_ => unreachable!(),
            _ => ArmInst::None,
        }
    }




    /// Decode a load/store misc. instruction.
    #[inline(always)]
    fn from_u32_lsmisc(x: u32) -> ArmInst {
        match get_decode_bits_lo!(x) {
            0b1001 => {
                match get_b!(x) {
                    false => ArmInst::Swp,
                    true => ArmInst::Swpb,
                }
            }
            0b1011 => {
                match (bit!(x, 22), get_l!(x)) {
                    (true, false) => ArmInst::StrhImm,
                    (true, true) => ArmInst::LdrhImm,
                    (false, false) => ArmInst::StrhReg,
                    (false, true) => ArmInst::LdrhReg,
                }
                //if bit!(x, 22) { return ArmInst::LsHalfImm; }
                //else { return ArmInst::LsHalfReg; }
            },
            0b1101 | 0b1110 | 0b1111 => {
                let tbl = (bit!(x, 20), bit!(x, 22), bit!(x, 5));
                return match tbl {
                    (true, false, false) => ArmInst::LdrsbReg,
                    (true, false, true) => ArmInst::LdrshReg,
                    (true, true, false) => ArmInst::LdrsbImm,
                    (true, true, true) => ArmInst::LdrshImm,

                    (false, false, false) => ArmInst::LdrdReg,
                    (false, false, true) => ArmInst::StrdReg,
                    (false, true, false) => ArmInst::LdrdImm,
                    (false, true, true) => ArmInst::StrdImm,
                }
            },
            _ => unreachable!("Instruction {:08x} {:032b}", x, x),
        }
    }

    #[inline(always)]
    fn from_u32_mul(x: u32) -> ArmInst {
        match (bit!(x, 23), bit!(x, 22), bit!(x, 21)) {
            (false, false, false) => return ArmInst::Mul,
            (false, false, true) => return ArmInst::Mla,

            (true, false, false) => return ArmInst::Umull,
            (true, false, true) => return ArmInst::Umlal,

            (true, true, false) => return ArmInst::Smull,
            (true, true, true) => return ArmInst::Smlal,
            _ => ArmInst::None,
            //_ => unreachable!("{:08x} ({:04x})", x, to_dec!(x)),
        }
    }

    pub fn from_u32(x: u32) -> Self { 
    let instr = match get_group!(x) {
        0b000 => { 
            if is_valid_multiply_instr!(x) { 
                return ArmInst::from_u32_mul(x); 
            }
            if is_valid_lsmisc_instr!(x) { 
                return ArmInst::from_u32_lsmisc(x); 
            }
            if is_valid_control_instr!(x) { 
                return ArmInst::from_u32_ctrl(x); 
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
        0b010 => {
            match (get_l!(x), get_b!(x)) {
                (false, false) => ArmInst::StrImm,
                (false, true) => ArmInst::StrbImm,
                (true, false) => ArmInst::LdrImm,
                (true, true) => ArmInst::LdrbImm,
            }
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
        0b100 => {
            match (get_l!(x), get_p!(x), get_u!(x)) {
                (false, false, false) => ArmInst::Stmda,
                (false, false, true) => ArmInst::Stmia,
                (false, true, false) => ArmInst::Stmdb,
                (false, true, true) => ArmInst::Stmib,

                (true, false, false) => ArmInst::Ldmda,
                (true, false, true) => ArmInst::Ldmia,
                (true, true, false) => ArmInst::Ldmdb,
                (true, true, true) => ArmInst::Ldmib,
            }
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
    };
    instr
    }
}

