
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use armdecode::*;

use armbf::traits::InstBits;
use armbf::newtype::*;
use armbf::inst::*;

use std::time::Instant;
use std::io::Read;

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

// One example; a user could implement a trait on all newtypes for running
// some code whenever we obtain a particular instruction

pub trait Interpretable<T: InstBits> { fn interpret(&self); }
impl Interpretable<DpShiftImmBf> for DpShiftImmBf { fn interpret(&self) { } }
impl Interpretable<DpShiftRegBf> for DpShiftRegBf { fn interpret(&self) { } }
impl Interpretable<DpRotImmBf> for DpRotImmBf { fn interpret(&self) { } }

fn interpret(op: &ArmInst) {
    match op {
        ArmInst::DpShiftReg(bf) => bf.interpret(),
        ArmInst::DpShiftImm(bf) => bf.interpret(),
        ArmInst::DpRotImm(bf) => bf.interpret(),
        _ => {},
    }
}

// Read some code, then decode/interpret it

fn main() {
    let mut buf = Vec::<u8>::new();
    let mut file = std::fs::File::open("testsuite/arm_test.bin").unwrap();
    file.read_to_end(&mut buf).unwrap();
    let databuf = make_u32_buf(&buf);

    run(&databuf);
}

fn run(databuf: &Vec<u32>) {
    let mut decoded = 0;
    while decoded < 100_000_000 {
        for val in databuf.iter() {
            let instr = decode(*val);
            interpret(&instr);
            decoded += 1;
            //println!("{:x?}", instr);
        }
    }
}
