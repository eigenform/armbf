//! Generic implementation of an ARMv5 lookup table.

use std::marker::Copy;
use crate::inst::*;

/// An ARMv5 lookup table.
pub struct ArmLut<T: ArmLutEntry> { pub data: [T; 0x1000] }

/// Implemented on all types store-able by some ArmLut.
pub trait ArmLutEntry { 
    /// A map from ArmInst to some ArmLutEntry.
    fn from_inst(inst: ArmInst) -> Self;
}

/// Creates a new ArmLut for some T.
///
/// The details of how to obtain an entry T are left to the user.
pub fn make_arm_lut<T: ArmLutEntry + Copy>(default_entry: T) -> ArmLut<T> {
    let mut lut = ArmLut { data: [default_entry; 0x1000] };
    for i in 0..0x1000 {
        let inst: u32 = ((i & 0x0ff0) << 16) | ((i & 0x000f) << 4);
        lut.data[i as usize] = T::from_inst(ArmInst::decode(inst));
    }
    lut
}

/// A Thumb lookup table.
pub struct ThumbLut<T: ThumbLutEntry> { pub data: [T; 0x0800] }

/// Implemented on all types store-able by some ThumbLut.
pub trait ThumbLutEntry {
    fn from_inst(inst: ThumbInst) -> Self;
}

/// Create a new ThumbLut for some T.
pub fn make_thumb_lut<T: ThumbLutEntry + Copy>(default_entry: T) -> ThumbLut<T> {
    let mut lut = ThumbLut { data: [default_entry; 0x0800] };
    for i in 0..0x800 {
        let inst: u16 = i << 5;
        lut.data[i as usize] = T::from_inst(ThumbInst::decode(inst));
    }
    lut
}
