//! Defines a set of decode-able ARM instructions.

use crate::newtype::*;

/// Unique types/kinds of instructions.
///
/// The inner type is some transparent wrapper type around a u32.
/// Users will need to pattern match against this in order to use it.
#[derive(Debug)]
pub enum ArmInst {
    // Undefined instruction
    None,

    // Control instructions
    MrsReg, 
    MrsImm, 
    Msr,
    Bx, 
    Blx, 
    Bxj, 
    Clz,
    Qadd, 
    Qsub, 
    QdAdd, 
    QdSub,
    Smla,
    Smlaw, 
    Smulw, 
    Smlal, 
    Smul,
    Bkpt,

    // Misc. load/store instructions
    Swp, 
    Swpb,
    LdrhImm, 
    LdrhReg,
    StrdLdrdImm, 
    StrdLdrdReg,
    StrhLdrhReg, 
    StrhLdrhImm,
    LdrshImm, 
    LdrshReg,
    LdrsbImm, 
    LdrsbReg,

    // Multiply instructions
    MulMla, 
    UmulUmla,

    // Data-processing instructions
    DpShiftReg(DpShiftRegBf), 
    DpShiftImm(DpShiftImmBf), 
    DpRotImm(DpRotImmBf),
    
    // Load/store instructions
    LsShift, 
    LsImm, 
    LsMulti,

    // Branching instructions
    Branch,

    // Coprocessor instructions
    CoprocLs, 
    CoprocDp, 
    CoprocRt, 

    // Software interupts
    Swi,
}
impl Default for ArmInst { fn default() -> Self { ArmInst::None } }



