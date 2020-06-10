//! Defines a set of decode-able ARM instructions.

use crate::newtype::*;

/// Unique types/kinds of ARM instructions.
///
/// Each variant in this enum has a corresponding inner newtype which wraps
/// a u32. Each newtype derives a set of traits which describe the valid set
/// of bitfields that one might want when decoding the instruction.

#[derive(Debug)]
pub enum ArmInst {
    // Undefined instruction
    None,

    // Control instructions (status register)
    MrsReg(StatusBf), 
    MrsImm(StatusBf), 
    Msr(StatusBf),

    // Control instructions (branch and exchange)
    Bx(BxBf), 
    BlxReg(BranchBf), 
    BlxImm(BranchBf),
    Bxj, 

    // Control instructions (saturated add/sub)
    Qadd(SatBf), 
    Qsub(SatBf), 
    QdAdd(SatBf), 
    QdSub(SatBf),

    // Control instructions (signed multiply)
    Smla(SmulBf),
    Smlaw(SmulBf), 
    Smulw(SmulBf), 
    Smlal(SmulBf), 
    Smul(SmulBf),

    // Control instructions (other)
    Clz(ClzBf),
    Bkpt(BkptBf),

    // Misc. load/store instructions (swap byte)
    Swp(SwpBf), 

    // Misc. load/store instructions (load/store halfword/doubleword)
    StrdLdrdImm(StrdLdrdImmBf), 
    StrdLdrdReg(StrdLdrdRegBf),
    StrhLdrhReg(StrhLdrhRegBf), 
    StrhLdrhImm(StrhLdrhImmBf),

    // Misc. load/store instructions (load signed byte/halfword)
    LdrshImm(LdrshImmBf), 
    LdrshReg(LdrshRegBf),
    LdrsbImm(LdrsbImmBf), 
    LdrsbReg(LdrsbRegBf),

    // Multiply instructions
    MulMla(MulBf), 
    UmulUmla(MulBf),

    // Data-processing instructions
    DpShiftReg(DpShiftRegBf), 
    DpShiftImm(DpShiftImmBf), 
    DpRotImm(DpRotImmBf),
    
    // Load/store instructions
    LsShift(LsShiftBf), 
    LsImm(LsImmBf), 
    LsMulti(LsMultiBf),

    // Branching instructions
    Branch(BranchBf),

    // Coprocessor instructions
    CoprocLs(CoprocBf), 
    CoprocDp(CoprocBf), 
    CoprocRt(CoprocBf), 

    // Software interrupts
    Swi(SwiBf),
}
impl Default for ArmInst { fn default() -> Self { ArmInst::None } }



