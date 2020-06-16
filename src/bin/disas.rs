//! Naive disassembler for flat binaries.

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use armdecode::*;
use armdecode::disas;

use disas::{LutFunc, undef_instr};

use armbf::newtype::*;
use armbf::inst::*;
use armbf::lut::*;

use std::time::Instant;
use std::io::Read;

extern crate rand;
use rand::prelude::*;

/// Convert a Vec<u8> into a Vec<u32> (in big-endian representation).
pub fn make_u32_buf(src_buf: &Vec<u8>) -> Vec<u32> {
    let mut dst_buf = Vec::<u32>::new();
    for val in src_buf.chunks(4) {
        let mut x = 0x0000_0000u32;
        x |= (val[0] as u32) << 24;
        x |= (val[1] as u32) << 16;
        x |= (val[2] as u32) << 8;
        x |= val[3] as u32;
        dst_buf.push(x);
    }
    dst_buf
}

/// Read a file with some code into some buffer, then disassemble it.
fn main() {
    let mut buf = Vec::<u8>::new();
    let mut file = std::fs::File::open("testsuite/arm_decode_test.bin")
        .unwrap();

    file.read_to_end(&mut buf).unwrap();
    let mut dbuf = make_u32_buf(&buf);
    disassemble(&dbuf);
}


/// Iterate through a buffer and decode/disassemble each value.
fn disassemble(dbuf: &Vec<u32>) {
    let mut offset = 0x0u32;
    let mut lut = CreateArmLut::<LutFunc>(LutFunc(undef_instr));
    println!("LUT is {:?}b", std::mem::size_of_val(&lut));

    let start = Instant::now();
    for val in dbuf.iter() {

        let idx = (((val >> 16) & 0x0ff0) | ((val >> 4) & 0x000f)) as usize;
        let disas_str = lut.data[idx].0(&val);
        println!("{:04x}:\t {:08x} {:04x}\t {}", offset, val, 
            ((val >> 16) & 0x0ff0) | ((val >> 4) & 0x000f), disas_str);
        offset += 4;
    }
    let dur = start.elapsed();
    let mdips = ((1f64 / dur.as_secs_f64()) * dbuf.len() as f64) / 1_000_000f64;

    println!("Disassembled {} instrs in {:?} (~{:.4}Mdips)", 
        dbuf.len(), dur, mdips);
}

