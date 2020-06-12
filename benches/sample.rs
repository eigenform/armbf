#![feature(test)]

extern crate test;
use armdecode;
use test::{black_box, Bencher};
use std::fs::File;
use std::io::Read;
use std::convert::TryFrom;

// Flat binary with some code to test decoding.
const TEST_FILE: &'static str = "testsuite/arm_decode_test.bin";

// You'll want to tune this to the size of the input.
// Otherwise, we'd be testing against dynamic allocations not-on-the-stack.
const IBUF_LEN: usize = 512;

#[bench]
fn arm_inst_space(b: &mut Bencher) { 
    // Read a file
    let mut file_buf = Vec::<u8>::new();
    let mut file = File::open(TEST_FILE).unwrap();
    let file_len = std::fs::metadata(TEST_FILE).unwrap().len();
    file.read(&mut file_buf).unwrap();

    // Copy into an array on the stack
    let mut ibuf = [0u32; 512];
    for (idx, val) in file_buf.chunks(4).enumerate() {
        let arr = <[u8;4]>::try_from(val).unwrap();
        let x = u32::from_be_bytes(arr);
        ibuf[idx] = x;
    }
    
    // Benchmark
    b.iter(|| 
        for val in ibuf.iter() { 
            let inst = armdecode::decode(*val);
        }
    );
}

