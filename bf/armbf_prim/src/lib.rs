//! Macro library for pulling bits out of numbers, for working with ARM
//! instructions.
//!
//! There are a bunch of macros here that armbf_derive depends on in order to 
//! actually implement the traits corresponding to bitfields. Ideally, it'd be
//! conceptually parsimonious to export all of these from armbf_derive, but 
//! apparently you can't do that in proc_macro crates.
//!

// Constants representing various bitmasks and matching values; used to 
// disambiguate certain groups of ARM instructions.

/// Decode bits.
pub const DECODE_BITS_MASK:     u32 = 0b0000_11111111_000000000000_1111_0000;

/// Mask for multiply instructions.
pub const MULTIPLY_MASK:        u32 = 0b0000_11110000_000000000000_1111_0000;

/// Match for multiply instructions.
pub const MULTIPLY_MATCH:       u32 = 0b0000_00000000_000000000000_1001_0000;

/// Mask for control instructions.
pub const CONTROL_MASK:         u32 = 0b0000_11011001_000000000000_0000_0000;

/// Match for control instructions.
pub const CONTROL_MATCH:        u32 = 0b0000_00010000_000000000000_0000_0000;

/// Mask for load/store miscellaneous instructions.
pub const LSMISC_MASK:          u32 = 0b0000_11100000_000000000000_1001_0000;

/// Match for load/store miscellaneous instructions.
pub const LSMISC_MATCH:         u32 = 0b0000_00000000_000000000000_1001_0000;

/// Get a bit in some number.
#[macro_export]
macro_rules! bit { ($val:expr, $bit:expr) => { ($val & (1 << $bit)) != 0 }}


// ----------------------------------------------------------------------------
// Macros specific to decoding ARM instructions.
//

/// Check if this number is a valid control instruction.
#[macro_export]
macro_rules! is_valid_control_instr { ($val:expr) => {
    (
        (($val & CONTROL_MASK) == CONTROL_MATCH) && 
        !(
            (bit!($val, 25) == false) && 
            (bit!($val, 7) == true) && 
            (bit!($val, 4) == true)
        )
    )
}}

/// Check if this number is a valid load/store misc instruction.
#[macro_export]
macro_rules! is_valid_lsmisc_instr { ($val:expr) => {
    ($val & LSMISC_MASK) == LSMISC_MATCH
}}

/// Check if this number is a valid multiply instruction.
#[macro_export]
macro_rules! is_valid_multiply_instr { ($val:expr) => {
    ($val & MULTIPLY_MASK) == MULTIPLY_MATCH
}}


// ----------------------------------------------------------------------------
// Generic/top-level bitfields.
//

/// Get the condition code (bits 31-28).
#[macro_export]
macro_rules! get_multiply_op { ($val:expr) => {
    ($val & 0b0000_0000_1100_0000_0000_0000_0000_0000) >> 22
}}

/// Get the condition code (bits 31-28).
#[macro_export]
macro_rules! get_cond { ($val:expr) => {
    ($val & 0b1111_0000_0000_0000_0000_0000_0000_0000) >> 28
}}

/// Get the instruction group (bits 27-25).
#[macro_export]
macro_rules! get_group { ($val:expr) => {
    ($val & 0b0000_1110_0000_0000_0000_0000_0000_0000) >> 25
}}

/// Get the high decode bits 27-20.
#[macro_export]
macro_rules! get_decode_bits_hi { ($val:expr) => { 
    ($val & 0b0000_1111_1111_0000_0000_0000_0000_0000) >> 20
}}

/// Get the low decode bits 7-4.
#[macro_export]
macro_rules! get_decode_bits_lo { ($val:expr) => { 
    ($val & 0b0000_0000_0000_0000_0000_0000_1111_0000) >> 4
}}


// ----------------------------------------------------------------------------
// Data-processing bitfields
//

/// Get the data processing opcode (bits 24-21).
#[macro_export]
macro_rules! get_opcd { ($val:expr) => {
    ($val & 0b0000_0001_1110_0000_0000_0000_0000_0000) >> 21
}}

/// Get the S bit (bit 20).
#[macro_export]
macro_rules! get_s { ($val:expr) => {
    ($val & 0b0000_0000_0001_0000_0000_0000_0000_0000) != 0
}}


// ----------------------------------------------------------------------------
// Register bitfields
//

/// Get the rN field (bits 19-16).
#[macro_export]
macro_rules! get_rn { ($val:expr) => {
    ($val & 0b0000_0000_0000_1111_0000_0000_0000_0000) >> 16
}}

/// Get the rD field (bits 15-12).
#[macro_export]
macro_rules! get_rd { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_1111_0000_0000_0000) >> 12
}}

/// Get the rS field (bits 11-8).
#[macro_export]
macro_rules! get_rs { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_1111_0000_0000) >> 8
}}

/// Get the rM field (bits 3-0).
#[macro_export]
macro_rules! get_rm { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_0000_0000_1111)
}}


// ----------------------------------------------------------------------------
// Immediate bitfields
//

#[macro_export]
macro_rules! get_imm4 { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_0000_0000_1111)
}}

/// Get the imm8 field (bits 7-0).
#[macro_export]
macro_rules! get_imm8 { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_0000_1111_1111)
}}

/// Get the imm12 field (bits 11-0).
#[macro_export]
macro_rules! get_imm12 { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_1111_1111_1111)
}}

#[macro_export]
macro_rules! get_imm12_hi { ($val:expr) => {
    ($val & 0b0000_0000_0000_1111_1111_1111_0000_0000) >> 8
}}

/// Get the imm24 field (bits 23-0).
#[macro_export]
macro_rules! get_imm24 { ($val:expr) => {
    ($val & 0b0000_0000_1111_1111_1111_1111_1111_1111)
}}

#[macro_export]
macro_rules! get_off_hi { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_1111_0000_0000)
}}

#[macro_export]
macro_rules! get_off_lo { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_0000_0000_1111)
}}



// ----------------------------------------------------------------------------
// Shifter/Rotate bitfields
//

/// Get the rotate immediate (bits 11-8).
#[macro_export]
macro_rules! get_rot_imm { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_1111_0000_0000) >> 8
}}

/// Get the shifter immediate (bits 11-7).
#[macro_export]
macro_rules! get_shift_imm { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_1111_1000_0000) >> 7
}}

/// Get the shift type (bits 6-5).
#[macro_export]
macro_rules! get_shift { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_0000_0110_0000) >> 5
}}


// ----------------------------------------------------------------------------
// Load/store bitfields
//

/// Get the P bit (bit 24).
#[macro_export]
macro_rules! get_p { ($val:expr) => {
    ($val & 0b0000_0001_0000_0000_0000_0000_0000_0000) != 0
}}

/// Get the U bit (bit 23).
#[macro_export]
macro_rules! get_u { ($val:expr) => {
    ($val & 0b0000_0000_1000_0000_0000_0000_0000_0000) != 0
}}

/// Get the B bit (bit 22).
#[macro_export]
macro_rules! get_b { ($val:expr) => {
    ($val & 0b0000_0000_0100_0000_0000_0000_0000_0000) != 0
}}

/// Get the W bit (bit 21).
#[macro_export]
macro_rules! get_w { ($val:expr) => {
    ($val & 0b0000_0000_0010_0000_0000_0000_0000_0000) != 0
}}

/// Get the L bit (bit 20).
#[macro_export]
macro_rules! get_l { ($val:expr) => {
    ($val & 0b0000_0000_0001_0000_0000_0000_0000_0000) != 0
}}


// ----------------------------------------------------------------------------
// Coprocessor bitfields
//

/// Get the coprocessor number (bits 11-8).
#[macro_export]
macro_rules! get_cp_num { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_1111_0000_0000) >> 8
}}

/// Get the coprocessor opcode 1 (bits 23-20).
#[macro_export]
macro_rules! get_cp_opcd1 { ($val:expr) => {
    ($val & 0b0000_0000_1111_0000_0000_0000_0000_0000) >> 20
}}

/// Get the coprocessor opcode 1 (bits 23-21, specific to coproc rt).
#[macro_export]
macro_rules! get_cp_opcd1_rt { ($val:expr) => {
    ($val & 0b0000_0000_1110_0000_0000_0000_0000_0000) >> 21
}}

/// Get the coprocessor opcode 2 (bits 7-5).
#[macro_export]
macro_rules! get_cp_opcd2 { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_0000_1110_0000) >> 5
}}

/// Get the crN field (bits 19-16).
#[macro_export]
macro_rules! get_crn { ($val:expr) => { get_rn!($val) }}

/// Get the crD field (bits 15-12).
#[macro_export]
macro_rules! get_crd { ($val:expr) => { get_rd!($val) }}

/// Get the crM field (bits 3-0).
#[macro_export]
macro_rules! get_crm { ($val:expr) => { get_rm!($val) }}

/// Get the N bit (bit 22, specific to coprocessor load/store instructions).
#[macro_export]
macro_rules! get_n { ($val:expr) => { get_b!($val) }}



// ----------------------------------------------------------------------------
// Control bitfields
//

/// Get the "control instruction opcode" (bits 7-4).
#[macro_export]
macro_rules! get_control_opcd { ($val:expr) => { 
    ($val & 0b0000_0000_0000_0000_0000_0000_1111_0000) >> 4
}}

#[macro_export]
macro_rules! get_lsmisc_op1 { ($val:expr) => { 
    ($val & 0b0000_0000_0000_0000_0000_0000_0110_0000) >> 5
}}

#[macro_export]
macro_rules! get_sat_addsub_op { ($val:expr) => { 
    ($val & 0b0000_0000_0110_0000_0000_0000_0000_0000) >> 21
}}

#[macro_export]
macro_rules! get_signed_mul_op { ($val:expr) => { 
    ($val & 0b0000_0000_0110_0000_0000_0000_0000_0000) >> 21
}}

/// Get the field_mask value (bits 19-16, for msr instructions).
#[macro_export]
macro_rules! get_field_mask { ($val:expr) => { get_rn!($val) }}


// ----------------------------------------------------------------------------
// Multiply bitfields
//

#[macro_export]
macro_rules! get_rd_hi { ($val:expr) => { get_rn!($val) }}

#[macro_export]
macro_rules! get_rd_lo { ($val:expr) => { get_rd!($val) }}

#[macro_export]
macro_rules! get_x { ($val:expr) => { 
    ($val & 0b0000_0000_0000_0000_0000_0000_0010_0000) != 0
}}

#[macro_export]
macro_rules! get_y { ($val:expr) => { 
    ($val & 0b0000_0000_0000_0000_0000_0000_0100_0000) != 0
}}

#[macro_export]
macro_rules! get_un { ($val:expr) => { 
    ($val & 0b0000_0000_0100_0000_0000_0000_0000_0000) != 0
}}

#[macro_export]
macro_rules! get_a { ($val:expr) => { 
    ($val & 0b0000_0000_0010_0000_0000_0000_0000_0000) != 0
}}



// ----------------------------------------------------------------------------
// Branch bitfields
//

/// Get the link bit (bit 24, specific to branching instructions).
#[macro_export]
macro_rules! get_link { ($val:expr) => { get_p!($val) }}


// ----------------------------------------------------------------------------
// Load/store multiple bitfields
//
 
/// Get the S bit (bit 22, specific to load/store multiple instructions).
#[macro_export]
macro_rules! get_s_multi { ($val:expr) => { get_b!($val) }}

/// Get the reglist field (bits 15-0).
#[macro_export]
macro_rules! get_reglist { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_1111_1111_1111_1111)
}}


