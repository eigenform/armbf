//! Macros for access to ARM instruction bitfields. 
//!
//! Note that there is no convention that prevents accidental misuse of these 
//! when writing something that actually parses out bitfields.

pub type DpShiftImm = u32;
pub trait DpShiftImmBf {
    fn cond(self) -> u32;
    fn opcd(self) -> u32;
    fn s(self) -> bool;
    fn rn(self) -> u32;
    fn rd(self) -> u32;
    fn shift_imm(self) -> u32;
    fn shift(self) -> u32;
    fn rm(self) -> u32;
}
impl DpShiftImmBf for DpShiftImm {
    #[inline(always)]
    fn cond(self) -> u32 { get_cond!(self) }
    #[inline(always)]
    fn opcd(self) -> u32 { get_opcd!(self) }
    #[inline(always)]
    fn s(self) -> bool { get_s!(self) }
    #[inline(always)]
    fn rn(self) -> u32 { get_rn!(self) }
    #[inline(always)]
    fn rd(self) -> u32 { get_rd!(self) }
    #[inline(always)]
    fn shift_imm(self) -> u32 { get_shift_imm!(self) }
    #[inline(always)]
    fn shift(self) -> u32 { get_shift!(self) }
    #[inline(always)]
    fn rm(self) -> u32 { get_rm!(self) }
}


pub type DpShiftReg = u32;
pub trait DpShiftRegBf {
    fn cond(self) -> u32;
    fn opcd(self) -> u32;
    fn s(self) -> bool;
    fn rn(self) -> u32;
    fn rd(self) -> u32;
    fn rs(self) -> u32;
    fn shift(self) -> u32;
    fn rm(self) -> u32;
}
impl DpShiftRegBf for DpShiftReg {
    #[inline(always)]
    fn cond(self) -> u32 { get_cond!(self) }
    #[inline(always)]
    fn opcd(self) -> u32 { get_opcd!(self) }
    #[inline(always)]
    fn s(self) -> bool { get_s!(self) }
    #[inline(always)]
    fn rn(self) -> u32 { get_rn!(self) }
    #[inline(always)]
    fn rd(self) -> u32 { get_rd!(self) }
    #[inline(always)]
    fn rs(self) -> u32 { get_rs!(self) }
    #[inline(always)]
    fn shift(self) -> u32 { get_shift!(self) }
    #[inline(always)]
    fn rm(self) -> u32 { get_rm!(self) }
}

pub type DpRotImm = u32;
pub trait DpRotImmBf {
    fn cond(self) -> u32;
    fn opcd(self) -> u32;
    fn s(self) -> bool;
    fn rn(self) -> u32;
    fn rd(self) -> u32;
    fn rot_imm(self) -> u32;
    fn imm8(self) -> u32;
}
impl DpRotImmBf for DpRotImm {
    #[inline(always)]
    fn cond(self) -> u32 { get_cond!(self) }
    #[inline(always)]
    fn opcd(self) -> u32 { get_opcd!(self) }
    #[inline(always)]
    fn s(self) -> bool { get_s!(self) }
    #[inline(always)]
    fn rn(self) -> u32 { get_rn!(self) }
    #[inline(always)]
    fn rd(self) -> u32 { get_rd!(self) }
    #[inline(always)]
    fn rot_imm(self) -> u32 { get_rot_imm!(self) }
    #[inline(always)]
    fn imm8(self) -> u32 { get_imm8!(self) }
}


//pub type LsImm = u32;
#[derive(LoadStoreFlags)]
pub struct LsImm(pub u32);
pub trait LsImmBf {
    fn cond(self) -> u32;
    fn rn(self) -> u32;
    fn rd(self) -> u32;
    fn imm12(self) -> u32;
}
impl LsImmBf for LsImm {
    #[inline(always)]
    fn cond(self) -> u32 { get_cond!(self) }
    #[inline(always)]
    fn rn(self) -> u32 { get_rn!(self) }
    #[inline(always)]
    fn rd(self) -> u32 { get_rd!(self) }
    #[inline(always)]
    fn imm12(self) -> u32 { get_imm12!(self) }
}


pub type LsShift = u32;
pub trait LsShiftBf {
    fn cond(self) -> u32;
    fn p(self) -> bool;
    fn u(self) -> bool;
    fn b(self) -> bool;
    fn w(self) -> bool;
    fn l(self) -> bool;
    fn rn(self) -> u32;
    fn rd(self) -> u32;
    fn shift_imm(self) -> u32;
    fn shift(self) -> u32;
    fn rm(self) -> u32;
}
impl LsShiftBf for LsShift {
    #[inline(always)]
    fn cond(self) -> u32 { get_cond!(self) }
    #[inline(always)]
    fn p(self) -> bool { get_p!(self) }
    #[inline(always)]
    fn u(self) -> bool { get_u!(self) }
    #[inline(always)]
    fn b(self) -> bool { get_b!(self) }
    #[inline(always)]
    fn w(self) -> bool { get_w!(self) }
    #[inline(always)]
    fn l(self) -> bool { get_l!(self) }
    #[inline(always)]
    fn rn(self) -> u32 { get_rn!(self) }
    #[inline(always)]
    fn rd(self) -> u32 { get_rd!(self) }
    #[inline(always)]
    fn shift_imm(self) -> u32 { get_shift_imm!(self) }
    #[inline(always)]
    fn shift(self) -> u32 { get_shift!(self) }
    #[inline(always)]
    fn rm(self) -> u32 { get_rm!(self) }
}


pub type LsMulti = u32;
pub trait LsMultiBf {
    fn cond(self) -> u32;
    fn p(self) -> bool;
    fn u(self) -> bool;
    fn b(self) -> bool;
    fn w(self) -> bool;
    fn l(self) -> bool;
    fn rn(self) -> u32;
    fn reglist(self) -> u32;
}
impl LsMultiBf for LsMulti {
    #[inline(always)]
    fn cond(self) -> u32 { get_cond!(self) }
    #[inline(always)]
    fn p(self) -> bool { get_p!(self) }
    #[inline(always)]
    fn u(self) -> bool { get_u!(self) }
    #[inline(always)]
    fn b(self) -> bool { get_b!(self) }
    #[inline(always)]
    fn w(self) -> bool { get_w!(self) }
    #[inline(always)]
    fn l(self) -> bool { get_l!(self) }
    #[inline(always)]
    fn rn(self) -> u32 { get_rn!(self) }
    #[inline(always)]
    fn reglist(self) -> u32 { get_reglist!(self) }
}


pub type Branch = u32;
pub trait BranchBf {
    fn cond(self) -> u32;
    fn link(self) -> bool;
    fn imm24(self) -> u32;
}
impl BranchBf for Branch {
    #[inline(always)]
    fn cond(self) -> u32 { get_cond!(self) }
    #[inline(always)]
    fn link(self) -> bool { get_link!(self) }
    #[inline(always)]
    fn imm24(self) -> u32 { get_imm24!(self) }
}












