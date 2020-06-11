//! Naive disassembler for flat binaries.

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use armdecode::*;

use armbf::newtype::*;
use armbf::inst::*;

use std::time::Instant;
use std::io::Read;

// Module with functions for formatting particular instructions
pub mod armfmt;

/// Interpret an instruction ArmInst, returning a unique string describing the 
/// disassembled form of the instruction.
fn get_disas_str(op: &ArmInst, offset: u32) -> String {
    match op {
        // Data-processing
        ArmInst::DpRotImm(bf)   => armfmt::dp_rot_imm(bf),
        ArmInst::DpShiftImm(bf) => armfmt::dp_shift_imm(bf),
        ArmInst::DpShiftReg(bf) => armfmt::dp_shift_reg(bf),
        
        // Load/store
        ArmInst::LsMulti(bf)    => armfmt::ls_multi(bf),
        ArmInst::LsImm(bf)      => armfmt::ls_imm(bf),
        ArmInst::LsHalfImm(bf)  => armfmt::ls_halfword_imm(bf),
        ArmInst::LsHalfReg(bf)  => armfmt::ls_halfword_reg(bf),

        // Branching
        ArmInst::Branch(bf)     => armfmt::branch(bf, offset),
        ArmInst::BlxImm(bf)     => armfmt::blx_imm(bf, offset),
        ArmInst::BlxReg(bf)     => armfmt::blx_reg(bf),
        ArmInst::Bx(bf)         => armfmt::bx(bf),

        // Saturated add/sub
        ArmInst::Qadd(bf)       => armfmt::qadd(bf),
        ArmInst::Qsub(bf)       => armfmt::qsub(bf),
        ArmInst::QdAdd(bf)      => armfmt::qdadd(bf),
        ArmInst::QdSub(bf)      => armfmt::qdsub(bf),

        // Signed multiply (extended)
        ArmInst::SmlaXy(bf)     => armfmt::smla_xy(bf),
        ArmInst::SmlalXy(bf)    => armfmt::smlal_xy(bf),
        ArmInst::SmulXy(bf)     => armfmt::smul_xy(bf),
        ArmInst::SmlawY(bf)     => armfmt::smlaw_y(bf),
        ArmInst::SmulwY(bf)     => armfmt::smulw_y(bf),

        // Multiply
        ArmInst::Mul(bf)        => armfmt::mul(bf),
        ArmInst::Mla(bf)        => armfmt::mla(bf),
        ArmInst::Umull(bf)      => armfmt::umull(bf),
        ArmInst::Umlal(bf)      => armfmt::umlal(bf),
        ArmInst::Smlal(bf)      => armfmt::smlal(bf),
        ArmInst::Smull(bf)      => armfmt::smull(bf),

        // Misc./Control
        ArmInst::Swi(bf)        => armfmt::swi(bf),
        ArmInst::Bkpt(bf)       => armfmt::bkpt(bf),
        ArmInst::Swp(bf)        => armfmt::swp(bf),
        ArmInst::Clz(bf)        => armfmt::clz(bf),

        // Status register 
        ArmInst::Mrs(bf)        => armfmt::mrs(bf),
        //ArmInst::MsrImm(bf)     => armfmt::msr_imm(bf),
        //ArmInst::MsrReg(bf)     => armfmt::msr_reg(bf),

        // Co-processor
        ArmInst::Mrc(bf)        => armfmt::mrc(bf),
        ArmInst::Mcr(bf)        => armfmt::mcr(bf),

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
    let databuf = make_u32_buf(&buf);
    disassemble(&databuf);
}

/// Iterate through a buffer and decode/disassemble each value.
fn disassemble(databuf: &Vec<u32>) {
    let mut offset = 0x0u32;
    for val in databuf.iter() {
        let instr = decode(*val);
        let disas_str = get_disas_str(&instr, offset);
        println!("{:04x}:\t {:08x}\t {}", offset, val, disas_str);
        offset += 4;
    }
}
