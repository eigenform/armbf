//! Macros for pulling bits out of numbers.
//!
//! There are a bunch of macros here that armbf_derive depends on in order to 
//! actually implement the traits corresponding to bitfields. 
//!
//! Ideally, it'd be conceptually parsimonious to export all of these from
//! armbf_derive, but apparently you can't do that in proc_macro crates.

#[macro_export]
/// Get a bit in some number.
macro_rules! bit { ($val:expr, $bit:expr) => {
    ($val & (1 << $bit)) != 0
}}

#[macro_export]
/// Get the condition code (bits 31-28).
macro_rules! get_multiply_op { ($val:expr) => {
    ($val & 0b0000_0000_1100_0000_0000_0000_0000_0000) >> 22
}}

#[macro_export]
/// Get the condition code (bits 31-28).
macro_rules! get_cond { ($val:expr) => {
    ($val & 0b1111_0000_0000_0000_0000_0000_0000_0000) >> 28
}}

#[macro_export]
/// Get the instruction group (bits 27-25).
macro_rules! get_group { ($val:expr) => {
    ($val & 0b0000_1110_0000_0000_0000_0000_0000_0000) >> 25
}}

#[macro_export]
/// Get the data processing opcode (bits 24-21).
macro_rules! get_opcd { ($val:expr) => {
    ($val & 0b0000_0001_1110_0000_0000_0000_0000_0000) >> 21
}}

#[macro_export]
/// Get the S bit (bit 20).
macro_rules! get_s { ($val:expr) => {
    ($val & 0b0000_0000_0001_0000_0000_0000_0000_0000) != 0
}}

#[macro_export]
/// Get the rN field (bits 19-16).
macro_rules! get_rn { ($val:expr) => {
    ($val & 0b0000_0000_0000_1111_0000_0000_0000_0000) >> 16
}}

#[macro_export]
/// Get the rD field (bits 15-12).
macro_rules! get_rd { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_1111_0000_0000_0000) >> 12
}}

#[macro_export]
/// Get the rS field (bits 11-8).
macro_rules! get_rs { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_1111_0000_0000) >> 8
}}

#[macro_export]
/// Get the rM field (bits 3-0).
macro_rules! get_rm { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_0000_0000_1111)
}}

#[macro_export]
/// Get the imm8 field (bits 7-0).
macro_rules! get_imm8 { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_0000_1111_1111)
}}

#[macro_export]
/// Get the imm12 field (bits 11-0).
macro_rules! get_imm12 { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_1111_1111_1111)
}}

#[macro_export]
/// Get the reglist field (bits 15-0).
macro_rules! get_reglist { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_1111_1111_1111_1111)
}}

#[macro_export]
/// Get the imm24 field (bits 23-0).
macro_rules! get_imm24 { ($val:expr) => {
    ($val & 0b0000_0000_1111_1111_1111_1111_1111_1111)
}}

#[macro_export]
/// Get the rotate immediate (bits 11-8).
macro_rules! get_rot_imm { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_1111_0000_0000) >> 8
}}

#[macro_export]
/// Get the shifter immediate (bits 11-7).
macro_rules! get_shift_imm { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_1111_1000_0000) >> 7
}}

#[macro_export]
/// Get the shift type (bits 6-5).
macro_rules! get_shift { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_0000_0110_0000) >> 5
}}

#[macro_export]
/// Get the P bit (bit 24).
macro_rules! get_p { ($val:expr) => {
    ($val & 0b0000_0001_0000_0000_0000_0000_0000_0000) != 0
}}

#[macro_export]
/// Get the link bit (bit 24, specific to branching instructions).
macro_rules! get_link { ($val:expr) => { get_p!($val) }}

 
#[macro_export]
/// Get the U bit (bit 23).
macro_rules! get_u { ($val:expr) => {
    ($val & 0b0000_0000_1000_0000_0000_0000_0000_0000) != 0
}}

#[macro_export]
/// Get the B bit (bit 22).
macro_rules! get_b { ($val:expr) => {
    ($val & 0b0000_0000_0100_0000_0000_0000_0000_0000) != 0
}}

#[macro_export]
/// Get the N bit (bit 22, specific to coprocessor load/store instructions).
macro_rules! get_n { ($val:expr) => { get_b!($val) }}

/// Get the S bit (bit 22, specific to load/store multiple instructions).
macro_rules! get_s_multi { ($val:expr) => { get_b!($val) }}


#[macro_export]
/// Get the W bit (bit 21).
macro_rules! get_w { ($val:expr) => {
    ($val & 0b0000_0000_0010_0000_0000_0000_0000_0000) != 0
}}

#[macro_export]
/// Get the L bit (bit 20).
macro_rules! get_l { ($val:expr) => {
    ($val & 0b0000_0000_0001_0000_0000_0000_0000_0000) != 0
}}

#[macro_export]
/// Get the coprocessor number (bits 11-8).
macro_rules! get_cp_num { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_1111_0000_0000) >> 8
}}

#[macro_export]
/// Get the coprocessor opcode 1 (bits 23-20).
macro_rules! get_cp_opcd1 { ($val:expr) => {
    ($val & 0b0000_0000_1111_0000_0000_0000_0000_0000) >> 20
}}

#[macro_export]
/// Get the coprocessor opcode 1 (bits 23-21, specific to coproc rt).
macro_rules! get_cp_opcd1_rt { ($val:expr) => {
    ($val & 0b0000_0000_1110_0000_0000_0000_0000_0000) >> 21
}}

#[macro_export]
/// Get the coprocessor opcode 2 (bits 7-5).
macro_rules! get_cp_opcd2 { ($val:expr) => {
    ($val & 0b0000_0000_0000_0000_0000_0000_1110_0000) >> 5
}}

#[macro_export]
/// Get the crN field (bits 19-16).
macro_rules! get_crn { ($val:expr) => { get_rn!($val) }}

#[macro_export]
/// Get the field_mask value (bits 19-16, for msr instructions).
macro_rules! get_field_mask { ($val:expr) => { get_rn!($val) }}

#[macro_export]
/// Get the crD field (bits 15-12).
macro_rules! get_crd { ($val:expr) => { get_rd!($val) }}

#[macro_export]
/// Get the crM field (bits 3-0).
macro_rules! get_crm { ($val:expr) => { get_rm!($val) }}

#[macro_export]
/// Get the "control instruction opcode" (bits 7-4).
macro_rules! get_control_opcd { ($val:expr) => { 
    ($val & 0b0000_0000_0000_0000_0000_0000_1111_0000) >> 4
}}

#[macro_export]
/// Get the high decode bits 27-20.
macro_rules! get_decode_bits_hi { ($val:expr) => { 
    ($val & 0b0000_1111_1111_0000_0000_0000_0000_0000) >> 20
}}

#[macro_export]
/// Get the low decode bits 7-4.
macro_rules! get_decode_bits_lo { ($val:expr) => { 
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






