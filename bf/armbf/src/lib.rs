//! Exposes wrapper types around unsigned 32-bit numeric representations of varous
//! types of ARM instructions.
//!
//! Each of the traits defined in this crate have a corresponding derive macro 
//! defined within the armbf_derive crate. The derive macros depend on all of
//! the generic bitfield getter/setter macros defined in the prim module.

#![allow(unused_macros)]
#![allow(unused_attributes)]

#![feature(arbitrary_enum_discriminant)]

//#![feature(trace_macros)]
//trace_macros!(true);

#[macro_use]

pub mod traits;
pub mod newtype;
pub mod fields;
pub mod inst;

