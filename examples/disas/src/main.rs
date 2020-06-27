//! Naive disassembler for flat binaries.

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate rand;
use rand::prelude::*;

use std::time::Instant;
use std::io::Read;

use armbf::newtype::*;
use armbf::inst::*;
use armbf::lut::*;

pub mod disas;

use disas::{LutFunc, ThumbLutFunc, undef_instr, undef_instr_thumb};

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

/// Convert a Vec<u8> to a Vec<16> (big-endian).
pub fn make_u16_buf(src_buf: &Vec<u8>) -> Vec<u16> {
    let mut dst_buf = Vec::<u16>::new();
    for val in src_buf.chunks(2) {
        let mut x = 0x0000u16;
        x |= (val[0] as u16) << 8;
        x |= val[1] as u16;
        dst_buf.push(x);
    }
    dst_buf
}


const ARM_FILE: &'static str = "../../testsuite/arm_decode_test.bin";
const THM_FILE: &'static str = "../../testsuite/thumb_decode_test.bin";

/// Read a file with some code into some buffer, then disassemble it.
fn main() {
    let mut arm_buf = Vec::<u8>::new();
    let mut arm_file = std::fs::File::open(ARM_FILE).unwrap();

    let mut thumb_buf = Vec::<u8>::new();
    let mut thumb_file = std::fs::File::open(THM_FILE).unwrap();


    arm_file.read_to_end(&mut arm_buf).unwrap();
    thumb_file.read_to_end(&mut thumb_buf).unwrap();
    let mut arm_code = make_u32_buf(&arm_buf);

    let mut thumb_code = make_u16_buf(&thumb_buf);

    disas_arm(&arm_code);
    disas_thumb(&thumb_code);
}


fn disas_arm(dbuf: &Vec<u32>) {
    let mut offset = 0x0u32;
    let mut lut = make_arm_lut::<LutFunc>(LutFunc(undef_instr));
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

fn disas_thumb(dbuf: &Vec<u16>) {
    let mut offset = 0x0u32;
    let mut lut = make_thumb_lut::<ThumbLutFunc>(ThumbLutFunc(undef_instr_thumb));
    println!("LUT is {:?}b", std::mem::size_of_val(&lut));

    let start = Instant::now();
    for val in dbuf.iter() {
        let idx = (val >> 5) as usize;
        let disas_str = lut.data[idx].0(&val);
        println!("{:04x}:\t {:04x} {}", offset, val, disas_str);
    }
    offset += 2;
    let dur = start.elapsed();
    let mdips = ((1f64 / dur.as_secs_f64()) * dbuf.len() as f64) / 1_000_000f64;
    println!("Disassembled {} instrs in {:?} (~{:.4}Mdips)", 
        dbuf.len(), dur, mdips);

}


