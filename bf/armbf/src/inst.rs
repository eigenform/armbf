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
    MrsReg, 
    MrsImm, 
    Msr,

    // Control instructions (branch and exchange)
    Bx(BxBf), 
    Blx, 
    Bxj, 

    // Control instructions (saturated add/sub)
    Qadd, 
    Qsub, 
    QdAdd, 
    QdSub,

    // Control instructions (signed multiply)
    Smla,
    Smlaw, 
    Smulw, 
    Smlal, 
    Smul,

    // Control instructions (other)
    Clz,
    Bkpt,

    // Misc. load/store instructions (swap byte)
    Swp, 
    Swpb,

    // Misc. load/store instructions (load halfword)
    LdrhImm, 
    LdrhReg,

    // Misc. load/store instructions (load/store halfword/doubleword)
    StrdLdrdImm, 
    StrdLdrdReg,
    StrhLdrhReg, 
    StrhLdrhImm,

    // Misc. load/store instructions (load signed byte/halfword)
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
    LsImm(LsImmBf), 
    LsMulti(LsMultiBf),

    // Branching instructions
    Branch,

    // Coprocessor instructions
    CoprocLs, 
    CoprocDp, 
    CoprocRt, 

    // Software interrupts
    Swi,
}
impl Default for ArmInst { fn default() -> Self { ArmInst::None } }



