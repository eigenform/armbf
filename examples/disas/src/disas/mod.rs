//! Functions returning strings with formatted ARM instructions.

use armbf::inst::*;
use armbf::lut::*;
use std::mem::transmute;

pub mod mul;
pub mod ctrl;
pub mod branch;
pub mod ls;
pub mod dp;
pub mod cp;

/// Inner type 
pub type DisFn = fn(x: &u32) -> String;

/// Newtype representing a function in the LUT.
#[derive(Copy, Clone)]
pub struct LutFunc(pub DisFn);

/// The undefined instruction handler.
pub fn undef_instr(x: &u32) -> String {
    let idx = ((*x >> 16) & 0x0ff0) | ((*x >> 4) & 0x000f);
    format!("No instruction; LUT index = {:04x}", idx)
}

/// A map from ArmInst to LutFunc.
impl ArmLutEntry for LutFunc {
    fn from_inst(inst: ArmInst) -> Self {

        // Convert to raw function pointer, then to DisFn.
        macro_rules! cfn { ($func:expr) => { unsafe {
            transmute::<*const fn(), DisFn>($func as *const fn())
        }}}

        match inst {
            ArmInst::MsrReg =>      LutFunc(cfn!(ctrl::msr_reg)),
            ArmInst::MsrImm =>      LutFunc(cfn!(ctrl::msr_imm)),
            ArmInst::Mrs =>         LutFunc(cfn!(ctrl::mrs)),
            ArmInst::Swi =>         LutFunc(cfn!(ctrl::svc)),
            ArmInst::Bkpt =>        LutFunc(cfn!(ctrl::bkpt)),
            ArmInst::Clz =>         LutFunc(cfn!(ctrl::clz)),
            ArmInst::Qadd =>        LutFunc(cfn!(ctrl::qadd)),
            ArmInst::Qsub =>        LutFunc(cfn!(ctrl::qsub)),
            ArmInst::QdAdd =>       LutFunc(cfn!(ctrl::qdadd)),
            ArmInst::QdSub =>       LutFunc(cfn!(ctrl::qdsub)),

            ArmInst::Mrc =>         LutFunc(cfn!(cp::mrc)),
            ArmInst::Mcr =>         LutFunc(cfn!(cp::mcr)),

            ArmInst::B =>           LutFunc(cfn!(branch::b)),
            ArmInst::Bl =>          LutFunc(cfn!(branch::bl)),
            ArmInst::Bx =>          LutFunc(cfn!(branch::bx)),
            ArmInst::BlxReg =>      LutFunc(cfn!(branch::blx_reg)),
            ArmInst::BlxImm =>      LutFunc(cfn!(branch::blx_imm)),

            ArmInst::LdrsbReg =>    LutFunc(cfn!(ls::ldrsb_reg)),
            ArmInst::LdrshReg =>    LutFunc(cfn!(ls::ldrsh_reg)),
            ArmInst::LdrsbImm =>    LutFunc(cfn!(ls::ldrsb_imm)),
            ArmInst::LdrshImm =>    LutFunc(cfn!(ls::ldrsh_imm)),
            ArmInst::StrdReg =>     LutFunc(cfn!(ls::strd_reg)),
            ArmInst::LdrdReg =>     LutFunc(cfn!(ls::ldrd_reg)),
            ArmInst::StrdImm =>     LutFunc(cfn!(ls::strd_imm)),
            ArmInst::LdrdImm =>     LutFunc(cfn!(ls::ldrd_imm)),
            ArmInst::StrhImm =>     LutFunc(cfn!(ls::strh_imm)),
            ArmInst::LdrhImm =>     LutFunc(cfn!(ls::ldrh_imm)),
            ArmInst::StrhReg =>     LutFunc(cfn!(ls::strh_reg)),
            ArmInst::LdrhReg =>     LutFunc(cfn!(ls::ldrh_reg)),
            ArmInst::Stmia =>       LutFunc(cfn!(ls::stmia)),
            ArmInst::Stmib =>       LutFunc(cfn!(ls::stmdb)),
            ArmInst::Stmda =>       LutFunc(cfn!(ls::stmda)),
            ArmInst::Stmdb =>       LutFunc(cfn!(ls::stmdb)),
            ArmInst::Ldmia =>       LutFunc(cfn!(ls::ldmia)),
            ArmInst::Ldmib =>       LutFunc(cfn!(ls::ldmib)),
            ArmInst::Ldmda =>       LutFunc(cfn!(ls::ldmda)),
            ArmInst::Ldmdb =>       LutFunc(cfn!(ls::ldmdb)),
            ArmInst::StrImm =>      LutFunc(cfn!(ls::str_imm)),
            ArmInst::LdrImm =>      LutFunc(cfn!(ls::ldr_imm)),
            ArmInst::StrbImm =>     LutFunc(cfn!(ls::strb_imm)),
            ArmInst::LdrbImm =>     LutFunc(cfn!(ls::ldrb_imm)),
            ArmInst::StrReg =>      LutFunc(cfn!(ls::str_reg)),
            ArmInst::LdrReg =>      LutFunc(cfn!(ls::ldr_reg)),
            ArmInst::StrbReg =>     LutFunc(cfn!(ls::strb_reg)),
            ArmInst::LdrbReg =>     LutFunc(cfn!(ls::ldrb_reg)),
            ArmInst::Swp =>         LutFunc(cfn!(ls::swp)),
            ArmInst::Swpb =>        LutFunc(cfn!(ls::swpb)),

            ArmInst::Mul =>         LutFunc(cfn!(mul::mul)),
            ArmInst::Mla =>         LutFunc(cfn!(mul::mla)),
            ArmInst::Umull =>       LutFunc(cfn!(mul::umull)),
            ArmInst::Umlal =>       LutFunc(cfn!(mul::umlal)),
            ArmInst::Smull =>       LutFunc(cfn!(mul::smull)),
            ArmInst::Smlal =>       LutFunc(cfn!(mul::smlal)),
            ArmInst::SmlaXy =>      LutFunc(cfn!(mul::smla_xy)),
            ArmInst::SmulwY =>      LutFunc(cfn!(mul::smulw_y)),
            ArmInst::SmlawY =>      LutFunc(cfn!(mul::smlaw_y)),
            ArmInst::SmlalXy =>     LutFunc(cfn!(mul::smlal_xy)),
            ArmInst::SmulXy =>      LutFunc(cfn!(mul::smul_xy)),

            ArmInst::AndRotImm =>   LutFunc(cfn!(dp::rot_imm_arith)),
            ArmInst::EorRotImm =>   LutFunc(cfn!(dp::rot_imm_arith)),
            ArmInst::SubRotImm =>   LutFunc(cfn!(dp::rot_imm_arith)),
            ArmInst::RsbRotImm =>   LutFunc(cfn!(dp::rot_imm_arith)),
            ArmInst::AddRotImm =>   LutFunc(cfn!(dp::rot_imm_arith)),
            ArmInst::AdcRotImm =>   LutFunc(cfn!(dp::rot_imm_arith)),
            ArmInst::SbcRotImm =>   LutFunc(cfn!(dp::rot_imm_arith)),
            ArmInst::RscRotImm =>   LutFunc(cfn!(dp::rot_imm_arith)),
            ArmInst::OrrRotImm =>   LutFunc(cfn!(dp::rot_imm_arith)),
            ArmInst::BicRotImm =>   LutFunc(cfn!(dp::rot_imm_arith)),
            ArmInst::TstRotImm =>   LutFunc(cfn!(dp::rot_imm_mov_cmp)),
            ArmInst::TeqRotImm =>   LutFunc(cfn!(dp::rot_imm_mov_cmp)),
            ArmInst::CmpRotImm =>   LutFunc(cfn!(dp::rot_imm_mov_cmp)),
            ArmInst::CmnRotImm =>   LutFunc(cfn!(dp::rot_imm_mov_cmp)),
            ArmInst::MovRotImm =>   LutFunc(cfn!(dp::rot_imm_mov_cmp)),
            ArmInst::MvnRotImm =>   LutFunc(cfn!(dp::rot_imm_mov_cmp)),

            ArmInst::AndShiftImm => LutFunc(cfn!(dp::shift_imm_arith)),
            ArmInst::EorShiftImm => LutFunc(cfn!(dp::shift_imm_arith)),
            ArmInst::SubShiftImm => LutFunc(cfn!(dp::shift_imm_arith)),
            ArmInst::RsbShiftImm => LutFunc(cfn!(dp::shift_imm_arith)),
            ArmInst::AddShiftImm => LutFunc(cfn!(dp::shift_imm_arith)),
            ArmInst::AdcShiftImm => LutFunc(cfn!(dp::shift_imm_arith)),
            ArmInst::SbcShiftImm => LutFunc(cfn!(dp::shift_imm_arith)),
            ArmInst::RscShiftImm => LutFunc(cfn!(dp::shift_imm_arith)),
            ArmInst::OrrShiftImm => LutFunc(cfn!(dp::shift_imm_arith)),
            ArmInst::BicShiftImm => LutFunc(cfn!(dp::shift_imm_arith)),
            ArmInst::TstShiftImm => LutFunc(cfn!(dp::shift_imm_cmp)),
            ArmInst::TeqShiftImm => LutFunc(cfn!(dp::shift_imm_cmp)),
            ArmInst::CmpShiftImm => LutFunc(cfn!(dp::shift_imm_cmp)),
            ArmInst::CmnShiftImm => LutFunc(cfn!(dp::shift_imm_cmp)),
            ArmInst::MovShiftImm => LutFunc(cfn!(dp::shift_imm_mov)),
            ArmInst::MvnShiftImm => LutFunc(cfn!(dp::shift_imm_mov)),

            ArmInst::AndShiftReg => LutFunc(cfn!(dp::shift_reg_arith)),
            ArmInst::EorShiftReg => LutFunc(cfn!(dp::shift_reg_arith)),
            ArmInst::SubShiftReg => LutFunc(cfn!(dp::shift_reg_arith)),
            ArmInst::RsbShiftReg => LutFunc(cfn!(dp::shift_reg_arith)),
            ArmInst::AddShiftReg => LutFunc(cfn!(dp::shift_reg_arith)),
            ArmInst::AdcShiftReg => LutFunc(cfn!(dp::shift_reg_arith)),
            ArmInst::SbcShiftReg => LutFunc(cfn!(dp::shift_reg_arith)),
            ArmInst::RscShiftReg => LutFunc(cfn!(dp::shift_reg_arith)),
            ArmInst::OrrShiftReg => LutFunc(cfn!(dp::shift_reg_arith)),
            ArmInst::BicShiftReg => LutFunc(cfn!(dp::shift_reg_arith)),
            ArmInst::TstShiftReg => LutFunc(cfn!(dp::shift_reg_cmp)),
            ArmInst::TeqShiftReg => LutFunc(cfn!(dp::shift_reg_cmp)),
            ArmInst::CmpShiftReg => LutFunc(cfn!(dp::shift_reg_cmp)),
            ArmInst::CmnShiftReg => LutFunc(cfn!(dp::shift_reg_cmp)),
            ArmInst::MovShiftReg => LutFunc(cfn!(dp::shift_reg_mov)),
            ArmInst::MvnShiftReg => LutFunc(cfn!(dp::shift_reg_mov)),

            _ => LutFunc(cfn!(undef_instr)),
        }
    }
}


