//! Exposes functions for decoding ARM instructions.

#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_macros)]
//#![allow(unused_imports)]

pub mod disas;

use armbf::newtype::*;
use armbf::inst::*;
use armbf_prim::*;


