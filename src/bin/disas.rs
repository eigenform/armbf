//! Naive disassembler for flat binaries.

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use armdecode::*;

use armbf::newtype::*;
use armbf::inst::*;

use std::time::Instant;
use std::io::Read;

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

/// Functions for disassembling particular instructions.
pub mod dis {
    use armbf::newtype::*;
    use armbf::traits::*;
    use armbf::fields::*;

    //pub fn dp_shift_imm(op: &DpShiftImmBf) -> String {
    //    format!("{} ",)
    //}

    pub fn ls_multi(op: &LsMultiBf) -> String {
        let mut reglist_str = std::string::String::new();
        for idx in 0..15 {
            if (op.reglist() & (1 << idx)) != 0 {
                reglist_str.push_str(
                    format!("{}, ", Register::from_u32(idx)).as_ref()
                );
            }
        }
        reglist_str.truncate(reglist_str.len() - 2);

        let rn = Register::from_u32(op.rn());
        let name = match (op.l(), &rn) {
            (true,  Register::sp) => "pop".to_string(),
            (false, Register::sp) => "push".to_string(),
            (true,  _) => format!("ldm {},", rn),
            (false, _) => format!("stm {},", rn),
        };

        format!("{} {{{}}}", name, reglist_str,)
    }



    pub fn dp_rot_imm(op: &DpRotImmBf) -> String {
        format!("{}{} {}, #{}",
            Opcode::from_u32(op.opcd()),
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rn()),
            op.imm8(),
        )
    }



    pub fn bx(op: &BxBf) -> String {
        format!("bx{} {}", 
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rm()))
    }


    pub fn dp_shift_imm(op: &DpShiftImmBf) -> String {
        format!("{} {}, {}", 
            Opcode::from_u32(op.opcd()),
            Register::from_u32(op.rd()),
            Register::from_u32(op.rn()),
        )
    }

    pub fn ls_imm(op: &LsImmBf) -> String {
        let name = match op.l() {
            true => "ldr",
            false => "str",
        };
        let width = match op.b() {
            true => "b",
            false => "",
        };
        format!("{}{} {}, [{}, #{}]", name, width, 
            Register::from_u32(op.rd()),
            Register::from_u32(op.rn()),
            op.imm12(),
        )
    }
}

fn get_disas_str(op: &ArmInst) -> String {
    match op {
        ArmInst::LsMulti(bf) =>  dis::ls_multi(bf),
        ArmInst::DpShiftImm(bf) =>  dis::dp_shift_imm(bf),
        ArmInst::DpRotImm(bf) =>  dis::dp_rot_imm(bf),
        ArmInst::LsImm(bf) =>    dis::ls_imm(bf),
        ArmInst::Bx(bf) =>    dis::bx(bf),
        _ => format!("{:?}",op).to_string(),
    }
}


// Read some code into a Vec<u32>, then just iterate and decode/interpret
// each instruction.

fn main() {
    let mut buf = Vec::<u8>::new();
    let mut file = std::fs::File::open("testsuite/arm_test.bin").unwrap();
    file.read_to_end(&mut buf).unwrap();
    let databuf = make_u32_buf(&buf);

    run(&databuf);
}

fn run(databuf: &Vec<u32>) {
    let mut offset = 0u32;
    for val in databuf.iter() {
        let instr = decode(*val);
        let disas_str = get_disas_str(&instr);
        println!("{:04x}:\t {:08x}\t {}", offset, val, disas_str);
        offset += 4;
    }
}
