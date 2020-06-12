//! Naive disassembler for flat binaries.

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use armdecode::*;
use armdecode::disas;

use armbf::newtype::*;
use armbf::inst::*;

use std::time::Instant;
use std::io::Read;

extern crate rand;
use rand::prelude::*;

// Module with functions for formatting particular instructions
//pub mod armfmt;

/// Interpret an instruction ArmInst, returning a unique string describing the 
/// disassembled form of the instruction.
fn get_disas_str(op: &ArmInst, offset: u32) -> String {
    match op {
        // Data-processing
        ArmInst::DpRotImm(bf)   => disas::dp_rot_imm(bf),
        ArmInst::DpShiftImm(bf) => disas::dp_shift_imm(bf),
        ArmInst::DpShiftReg(bf) => disas::dp_shift_reg(bf),
        
        // Load/store
        ArmInst::LsMulti(bf)    => disas::ls_multi(bf),
        ArmInst::LsImm(bf)      => disas::ls_imm(bf),
        ArmInst::LsHalfImm(bf)  => disas::ls_halfword_imm(bf),
        ArmInst::LsHalfReg(bf)  => disas::ls_halfword_reg(bf),

        // Branching
        ArmInst::Branch(bf)     => disas::branch(bf, offset),
        ArmInst::BlxImm(bf)     => disas::blx_imm(bf, offset),
        ArmInst::BlxReg(bf)     => disas::blx_reg(bf),
        ArmInst::Bx(bf)         => disas::bx(bf),

        // Saturated add/sub
        ArmInst::Qadd(bf)       => disas::qadd(bf),
        ArmInst::Qsub(bf)       => disas::qsub(bf),
        ArmInst::QdAdd(bf)      => disas::qdadd(bf),
        ArmInst::QdSub(bf)      => disas::qdsub(bf),

        // Signed multiply (extended)
        ArmInst::SmlaXy(bf)     => disas::smla_xy(bf),
        ArmInst::SmlalXy(bf)    => disas::smlal_xy(bf),
        ArmInst::SmulXy(bf)     => disas::smul_xy(bf),
        ArmInst::SmlawY(bf)     => disas::smlaw_y(bf),
        ArmInst::SmulwY(bf)     => disas::smulw_y(bf),

        // Multiply
        ArmInst::Mul(bf)        => disas::mul(bf),
        ArmInst::Mla(bf)        => disas::mla(bf),
        ArmInst::Umull(bf)      => disas::umull(bf),
        ArmInst::Umlal(bf)      => disas::umlal(bf),
        ArmInst::Smlal(bf)      => disas::smlal(bf),
        ArmInst::Smull(bf)      => disas::smull(bf),

        // Misc./Control
        ArmInst::Swi(bf)        => disas::swi(bf),
        ArmInst::Bkpt(bf)       => disas::bkpt(bf),
        ArmInst::Swp(bf)        => disas::swp(bf),
        ArmInst::Clz(bf)        => disas::clz(bf),

        // Status register 
        ArmInst::Mrs(bf)        => disas::mrs(bf),
        ArmInst::MsrImm(bf)     => disas::msr_imm(bf),
        ArmInst::MsrReg(bf)     => disas::msr_reg(bf),

        // Co-processor
        ArmInst::Mrc(bf)        => disas::mrc(bf),
        ArmInst::Mcr(bf)        => disas::mcr(bf),

        // No formatter defined
        _ => format!("{:?}",op).to_string(),
    }
}

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
    //let mut file = std::fs::File::open("testsuite/arm_test.bin").unwrap();
    let mut file = std::fs::File::open("testsuite/arm_decode_test.bin").unwrap();
    file.read_to_end(&mut buf).unwrap();
    let mut databuf = make_u32_buf(&buf);

    let start = Instant::now();
    disassemble(&databuf);
    let dur = start.elapsed();

    let mdips = ((1f64 / dur.as_secs_f64()) * databuf.len() as f64) / 1_000_000f64;
    println!("Decoded {} instrs in {:?} (~{:.4}Mdips)", databuf.len(), dur, mdips);
}

/// Iterate through a buffer and decode/disassemble each value.
fn disassemble(databuf: &Vec<u32>) {
    let mut offset = 0x0u32;
    for val in databuf.iter() {
        let instr = decode(*val);

        //let disas_str = get_disas_str(&instr, offset);
        //println!("{:04x}:\t {:08x}\t {}", offset, val, disas_str);

        offset += 4;
    }
}
