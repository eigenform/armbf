//! Functions returning strings with formatted ARM instructions.

use armbf::inst::*;
use std::mem::transmute;

pub mod mul;
pub mod ctrl;
pub mod branch;
pub mod ls;
pub mod dp;
pub mod cp;

/// 12-bit lookup table for ARM instructions.
pub struct ARMLookupTable {
    pub data: [fn(x: &u32) -> String; 0x1000],
}
impl ARMLookupTable {
    unsafe fn insert(&mut self, idx: usize, func: *const fn()) {
        self.data[idx] = transmute::<*const fn(), fn(x: &u32) -> String>(func);
    }

    /// Build a new lookup table.
    pub fn new() -> Self {
        let mut lut = ARMLookupTable {
            data: [undef_instr; 0x1000],
        };

        for i in 0..0x1000 {
            let inst: u32 = ((i & 0x0ff0) << 16) | ((i & 0x000f) << 4);
            unsafe {
                lut.insert(i as usize, ArmInst::from_u32(inst).to_func());
            }
        }
        lut
    }
}

pub fn undef_instr(x: &u32) -> String {
    let idx = ((*x >> 16) & 0x0ff0) | ((*x >> 4) & 0x000f);
    format!("No instruction; LUT index = {:04x}", idx)
}

/// To-be-implemented on ArmInst; maps instructions to functions.
pub trait LutFunc { fn to_func(self) -> *const fn(); }

/// Map instructions to particular functions.
impl LutFunc for ArmInst {
    fn to_func(self) -> *const fn() {

        macro_rules! cfn { ($func:expr) => { $func as *const fn() }}

        match self {
            ArmInst::MsrReg =>      cfn!(ctrl::msr_reg),
            ArmInst::MsrImm =>      cfn!(ctrl::msr_imm),
            ArmInst::Mrs =>         cfn!(ctrl::mrs),
            ArmInst::Swi =>         cfn!(ctrl::svc),
            ArmInst::Bkpt =>        cfn!(ctrl::bkpt),
            ArmInst::Clz =>         cfn!(ctrl::clz),
            ArmInst::Qadd =>        cfn!(ctrl::qadd),
            ArmInst::Qsub =>        cfn!(ctrl::qsub),
            ArmInst::QdAdd =>       cfn!(ctrl::qdadd),
            ArmInst::QdSub =>       cfn!(ctrl::qdsub),

            ArmInst::Mrc =>         cfn!(cp::mrc),
            ArmInst::Mcr =>         cfn!(cp::mcr),

            ArmInst::B =>           cfn!(branch::b),
            ArmInst::Bl =>          cfn!(branch::bl),
            ArmInst::Bx =>          cfn!(branch::bx),
            ArmInst::BlxReg =>      cfn!(branch::blx_reg),
            ArmInst::BlxImm =>      cfn!(branch::blx_imm),

            ArmInst::LdrsbReg =>    cfn!(ls::ldrsb_reg),
            ArmInst::LdrshReg =>    cfn!(ls::ldrsh_reg),
            ArmInst::LdrsbImm =>    cfn!(ls::ldrsb_imm),
            ArmInst::LdrshImm =>    cfn!(ls::ldrsh_imm),
            ArmInst::StrdReg =>     cfn!(ls::strd_reg),
            ArmInst::LdrdReg =>     cfn!(ls::ldrd_reg),
            ArmInst::StrdImm =>     cfn!(ls::strd_imm),
            ArmInst::LdrdImm =>     cfn!(ls::ldrd_imm),
            ArmInst::StrhImm =>     cfn!(ls::strh_imm),
            ArmInst::LdrhImm =>     cfn!(ls::ldrh_imm),
            ArmInst::StrhReg =>     cfn!(ls::strh_reg),
            ArmInst::LdrhReg =>     cfn!(ls::ldrh_reg),
            ArmInst::Stmia =>       cfn!(ls::stmia),
            ArmInst::Stmib =>       cfn!(ls::stmdb),
            ArmInst::Stmda =>       cfn!(ls::stmda),
            ArmInst::Stmdb =>       cfn!(ls::stmdb),
            ArmInst::Ldmia =>       cfn!(ls::ldmia),
            ArmInst::Ldmib =>       cfn!(ls::ldmib),
            ArmInst::Ldmda =>       cfn!(ls::ldmda),
            ArmInst::Ldmdb =>       cfn!(ls::ldmdb),
            ArmInst::StrImm =>      cfn!(ls::str_imm),
            ArmInst::LdrImm =>      cfn!(ls::ldr_imm),
            ArmInst::StrbImm =>     cfn!(ls::strb_imm),
            ArmInst::LdrbImm =>     cfn!(ls::ldrb_imm),
            ArmInst::StrReg =>      cfn!(ls::str_reg),
            ArmInst::LdrReg =>      cfn!(ls::ldr_reg),
            ArmInst::StrbReg =>     cfn!(ls::strb_reg),
            ArmInst::LdrbReg =>     cfn!(ls::ldrb_reg),
            ArmInst::Swp =>         cfn!(ls::swp),
            ArmInst::Swpb =>        cfn!(ls::swpb),

            ArmInst::Mul =>         cfn!(mul::mul),
            ArmInst::Mla =>         cfn!(mul::mla),
            ArmInst::Umull =>       cfn!(mul::umull),
            ArmInst::Umlal =>       cfn!(mul::umlal),
            ArmInst::Smull =>       cfn!(mul::smull),
            ArmInst::Smlal =>       cfn!(mul::smlal),
            ArmInst::SmlaXy =>      cfn!(mul::smla_xy),
            ArmInst::SmulwY =>      cfn!(mul::smulw_y),
            ArmInst::SmlawY =>      cfn!(mul::smlaw_y),
            ArmInst::SmlalXy =>     cfn!(mul::smlal_xy),
            ArmInst::SmulXy =>      cfn!(mul::smul_xy),

            ArmInst::AndRotImm =>   cfn!(dp::rot_imm_arith),
            ArmInst::EorRotImm =>   cfn!(dp::rot_imm_arith),
            ArmInst::SubRotImm =>   cfn!(dp::rot_imm_arith),
            ArmInst::RsbRotImm =>   cfn!(dp::rot_imm_arith),
            ArmInst::AddRotImm =>   cfn!(dp::rot_imm_arith),
            ArmInst::AdcRotImm =>   cfn!(dp::rot_imm_arith),
            ArmInst::SbcRotImm =>   cfn!(dp::rot_imm_arith),
            ArmInst::RscRotImm =>   cfn!(dp::rot_imm_arith),
            ArmInst::OrrRotImm =>   cfn!(dp::rot_imm_arith),
            ArmInst::BicRotImm =>   cfn!(dp::rot_imm_arith),
            ArmInst::TstRotImm =>   cfn!(dp::rot_imm_mov_cmp),
            ArmInst::TeqRotImm =>   cfn!(dp::rot_imm_mov_cmp),
            ArmInst::CmpRotImm =>   cfn!(dp::rot_imm_mov_cmp),
            ArmInst::CmnRotImm =>   cfn!(dp::rot_imm_mov_cmp),
            ArmInst::MovRotImm =>   cfn!(dp::rot_imm_mov_cmp),
            ArmInst::MvnRotImm =>   cfn!(dp::rot_imm_mov_cmp),

            ArmInst::AndShiftImm => cfn!(dp::shift_imm_arith),
            ArmInst::EorShiftImm => cfn!(dp::shift_imm_arith),
            ArmInst::SubShiftImm => cfn!(dp::shift_imm_arith),
            ArmInst::RsbShiftImm => cfn!(dp::shift_imm_arith),
            ArmInst::AddShiftImm => cfn!(dp::shift_imm_arith),
            ArmInst::AdcShiftImm => cfn!(dp::shift_imm_arith),
            ArmInst::SbcShiftImm => cfn!(dp::shift_imm_arith),
            ArmInst::RscShiftImm => cfn!(dp::shift_imm_arith),
            ArmInst::OrrShiftImm => cfn!(dp::shift_imm_arith),
            ArmInst::BicShiftImm => cfn!(dp::shift_imm_arith),
            ArmInst::TstShiftImm => cfn!(dp::shift_imm_cmp),
            ArmInst::TeqShiftImm => cfn!(dp::shift_imm_cmp),
            ArmInst::CmpShiftImm => cfn!(dp::shift_imm_cmp),
            ArmInst::CmnShiftImm => cfn!(dp::shift_imm_cmp),
            ArmInst::MovShiftImm => cfn!(dp::shift_imm_mov),
            ArmInst::MvnShiftImm => cfn!(dp::shift_imm_mov),

            ArmInst::AndShiftReg => cfn!(dp::shift_reg_arith),
            ArmInst::EorShiftReg => cfn!(dp::shift_reg_arith),
            ArmInst::SubShiftReg => cfn!(dp::shift_reg_arith),
            ArmInst::RsbShiftReg => cfn!(dp::shift_reg_arith),
            ArmInst::AddShiftReg => cfn!(dp::shift_reg_arith),
            ArmInst::AdcShiftReg => cfn!(dp::shift_reg_arith),
            ArmInst::SbcShiftReg => cfn!(dp::shift_reg_arith),
            ArmInst::RscShiftReg => cfn!(dp::shift_reg_arith),
            ArmInst::OrrShiftReg => cfn!(dp::shift_reg_arith),
            ArmInst::BicShiftReg => cfn!(dp::shift_reg_arith),
            ArmInst::TstShiftReg => cfn!(dp::shift_reg_cmp),
            ArmInst::TeqShiftReg => cfn!(dp::shift_reg_cmp),
            ArmInst::CmpShiftReg => cfn!(dp::shift_reg_cmp),
            ArmInst::CmnShiftReg => cfn!(dp::shift_reg_cmp),
            ArmInst::MovShiftReg => cfn!(dp::shift_reg_mov),
            ArmInst::MvnShiftReg => cfn!(dp::shift_reg_mov),

            _ => cfn!(undef_instr),
        }
    }
}

