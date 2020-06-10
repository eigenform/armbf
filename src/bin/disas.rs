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

    /// Sign extend to some number of bits
    #[inline(always)]
    pub fn sign_extend(x: i32, bits: i32) -> i32 {
        if ( (x >> (bits - 1)) & 1) != 0 {
            return x | !0 << bits
        }
        x
    }

    //pub fn dp_shift_imm(op: &DpShiftImmBf) -> String {
    //    format!("{} ",)
    //}


    //
    // Control instructions
    //

    pub fn swi(op: &SwiBf) -> String { format!("svc\t 0x{:08x}", op.imm24()) }
    pub fn bkpt(op: &BkptBf) -> String { 
        format!("bkpt\t 0x{:04x}", 
            ((op.imm12_hi() << 4) | op.imm4()) as u16
        )
    }

    pub fn qadd(op: &SatBf) -> String { format!("qadd{}\t {}, {}, {}",
            Cond::from_u32(op.cond()), Register::from_u32(op.rd()),
            Register::from_u32(op.rm()), Register::from_u32(op.rn()),
        )
    }
    pub fn qdadd(op: &SatBf) -> String { format!("qdadd{}\t {}, {}, {}",
            Cond::from_u32(op.cond()), Register::from_u32(op.rd()),
            Register::from_u32(op.rm()), Register::from_u32(op.rn()),
        )
    }
    pub fn qsub(op: &SatBf) -> String { format!("qsub{}\t {}, {}, {}",
            Cond::from_u32(op.cond()), Register::from_u32(op.rd()),
            Register::from_u32(op.rm()), Register::from_u32(op.rn()),
        )
    }
    pub fn qdsub(op: &SatBf) -> String { format!("qdsub{}\t {}, {}, {}",
            Cond::from_u32(op.cond()), Register::from_u32(op.rd()),
            Register::from_u32(op.rm()), Register::from_u32(op.rn()),
        )
    }

    //
    // Load/store instructions
    //

    pub fn ls_imm(op: &LsImmBf) -> String {
        let name = match op.l() { true => "ldr", false => "str", };
        let width = match op.b() { true => "b", false => "", };
        format!("{}{}\t {}, [{}, #{}]", 
            name, width, 
            Register::from_u32(op.rd()),
            Register::from_u32(op.rn()),
            op.imm12(),
        )
    }

    pub fn ls_multi(op: &LsMultiBf) -> String {
        let mut reglist_str = std::string::String::new();
        let rn = Register::from_u32(op.rn());

        for idx in 0..15 {
            if (op.reglist() & (1 << idx)) != 0 {
                reglist_str.push_str( format!("{}, ", 
                        Register::from_u32(idx)).as_ref()
                );
            }
        }
        reglist_str.truncate(reglist_str.len() - 2);

        let name = match (op.l(), &rn) {
            (true,  Register::sp) => "pop".to_string(),
            (false, Register::sp) => "push".to_string(),
            (true,  _) => format!("ldm {},", rn),
            (false, _) => format!("stm {},", rn),
        };

        format!("{}\t {{{}}}", name, reglist_str,)
    }

    pub fn swp(op: &SwpBf) -> String { 
        let name = if op.b() { "swpb" } else { "swp" };
        format!("{}\t {}, {}, [{}]", 
            name,
            Register::from_u32(op.rd()),
            Register::from_u32(op.rm()),
            Register::from_u32(op.rn()),
        )
    }

    pub fn ls_halfword_imm(op: &StrhLdrhImmBf) -> String {
        let name = if op.l() { "ldrh" } else { "strh" };
        let imm = if op.u() {
            ((op.off_hi() << 4) | op.off_lo()) as i32
        } else {
            (((op.off_hi() << 4) | op.off_lo()) as i32).wrapping_neg()
        };
        format!("{}{}\t {}, [{}, #{}]",
            name,
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rd()),
            Register::from_u32(op.rn()),
            imm,
        )
    }

    pub fn ls_halfword_reg(op: &StrhLdrhRegBf) -> String {
        let name = if op.l() { "ldrh" } else { "strh" };
        format!("{}{}\t {}, [{}, {}]",
            name,
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rd()),
            Register::from_u32(op.rn()),
            Register::from_u32(op.rm()),
        )
    }


    //
    // Data processing instructions
    //



    pub fn dp_rot_imm(op: &DpRotImmBf) -> String { 
        let opcd = Opcode::from_u32(op.opcd());
        let cond = Cond::from_u32(op.cond());
        let rn = Register::from_u32(op.rn());
        let rd = Register::from_u32(op.rd());
        let imm8 = op.imm8();

        return match opcd {
            Opcode::Cmp => format!("{}{}\t {}, #{}", opcd, cond, rn, imm8),
            Opcode::Cmn => format!("{}{}\t {}, #{}", opcd, cond, rn, imm8),
            Opcode::Mov => format!("{}{}\t {}, #{}", opcd, cond, rd, imm8),
            Opcode::Mvn => format!("{}{}\t {}, #{}", opcd, cond, rd, imm8),
            Opcode::Teq => format!("{}{}\t {}, #{}", opcd, cond, rn, imm8),
            Opcode::Tst => format!("{}{}\t {}, #{}", opcd, cond, rn, imm8),
            _ => format!("{}{}\t {}, {}, #{}", opcd, cond, rd, rn, imm8),
        };

    }

    pub fn dp_shift_imm(op: &DpShiftImmBf) -> String { 
        let opcd = Opcode::from_u32(op.opcd());
        let cond = Cond::from_u32(op.cond());
        let rn = Register::from_u32(op.rn());
        let rd = Register::from_u32(op.rd());
        let rm = Register::from_u32(op.rm());
        let shift_imm = op.shift_imm();
        let shift_type = ShifterType::from_u32(op.shift());

        match opcd {
            Opcode::Cmn => { format!("{}{}\t {}, {}", opcd, cond, rn, 
                format!("{}, {} #{}", rm, shift_type, shift_imm))
            },
            Opcode::Cmp => { format!("{}{}\t {}, {}", opcd, cond, rn, 
                format!("{}, {} #{}", rm, shift_type, shift_imm))
            },

            Opcode::Mov => { format!("{}{}\t {}, {}, #{}", 
                shift_type, cond, rd, rm, shift_imm)
            },

            Opcode::Mvn => { format!("{}{}\t {}, {}, {} #{}", 
                opcd, cond, rd, rm, shift_type, shift_imm)
            },

            Opcode::Teq => { format!("{}{}\t {}, {}", opcd, cond, rn, 
                format!("{}, {} #{}", rm, shift_type, shift_imm))
            },
            Opcode::Tst => { format!("{}{}\t {}, {}", opcd, cond, rn, 
                format!("{}, {} #{}", rm, shift_type, shift_imm))
            },
            _ => { format!("{}{}\t {}, {}, {} {} #{}", opcd, cond, rd, rn, 
                rm, shift_type, shift_imm)
            },
        }
    }

    pub fn dp_shift_reg(op: &DpShiftRegBf) -> String { 
        let opcd = Opcode::from_u32(op.opcd());
        let cond = Cond::from_u32(op.cond());
        let rn = Register::from_u32(op.rn());
        let rd = Register::from_u32(op.rd());
        let rm = Register::from_u32(op.rm());
        let rs = Register::from_u32(op.rs());
        let shift_type = ShifterType::from_u32(op.shift());

        match opcd {
            Opcode::Cmn => { format!("{}{}\t {}, {}", opcd, cond, rn, 
                format!("{}, {} {}", rm, shift_type, rs))
            },
            Opcode::Cmp => { format!("{}{}\t {}, {}", opcd, cond, rn, 
                format!("{}, {} {}", rm, shift_type, rs))
            },

            Opcode::Mov => { format!("{}{}\t {}, {}, {}", 
                shift_type, cond, rd, rm, rs)
            },

            Opcode::Mvn => { format!("{}{}\t {}, {}, {} {}", 
                opcd, cond, rd, rm, shift_type, rs)
            },

            Opcode::Teq => { format!("{}{}\t {}, {}", opcd, cond, rn, 
                format!("{}, {} {}", rm, shift_type, rs))
            },
            Opcode::Tst => { format!("{}{}\t {}, {}", opcd, cond, rn, 
                format!("{}, {} {}", rm, shift_type, rs))
            },
            _ => { format!("{}{}\t {}, {}, {} {} {}", opcd, cond, rd, rn, 
                rm, shift_type, rs)
            },
        }
    }






    //
    // Branching instructions
    //

    pub fn blx_imm(op: &BranchBf, offset: u32) -> String { 
        let imm24 = sign_extend(op.imm24() as i32, 24) << 2;
        let dest = (offset as i64) + (imm24 as i64) + 8;
        format!("blx{}\t {:04x}",
            Cond::from_u32(op.cond()),
            dest
        )
    }
    pub fn blx_reg(op: &BranchBf) -> String { 
        format!("blx\t {}", Register::from_u32(op.rm()))
    }

    pub fn bx(op: &BxBf) -> String { format!("bx{}\t {}", 
        Cond::from_u32(op.cond()), Register::from_u32(op.rm()))
    }
    pub fn branch(op: &BranchBf, offset: u32) -> String {
        let name = if op.link() { "bl" } else { "b" };
        let imm24 = sign_extend(op.imm24() as i32, 24) << 2;
        let dest = (offset as i64) + (imm24 as i64) + 8;
        format!("{}{}\t {:04x}",
            name, 
            Cond::from_u32(op.cond()),
            dest,
        )
    }

    //
    // Multiply instructions (extended)
    //

    pub fn smla_xy(op: &MulBf) -> String {
        let xy = match (op.x(), op.y()) {
            (false, false) => "bb", (false, true) => "bt",
            (true, false) => "tb", (true, true) => "tt",
        };
        format!("slma{}{}\t {}, {}, {}, {}", xy,
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rd_alt()),
            Register::from_u32(op.rm()),
            Register::from_u32(op.rs()),
            Register::from_u32(op.rn_alt()),
        )
    }

    pub fn smlal_xy(op: &MulBf) -> String {
        let xy = match (op.x(), op.y()) {
            (false, false) => "bb", (false, true) => "bt",
            (true, false) => "tb", (true, true) => "tt",
        };
        format!("slmal{}{} {}, {}, {}, {}", xy,
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rn_alt()),
            Register::from_u32(op.rd_alt()),
            Register::from_u32(op.rm()),
            Register::from_u32(op.rs()),
        )
    }

    pub fn smul_xy(op: &MulBf) -> String {
        let xy = match (op.x(), op.y()) {
            (false, false) => "bb", (false, true) => "bt",
            (true, false) => "tb", (true, true) => "tt",
        };
        format!("smul{}{}  {}, {}, {}", xy,
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rd_alt()),
            Register::from_u32(op.rm()),
            Register::from_u32(op.rs()),
        )
    }

    pub fn smlaw_y(op: &MulBf) -> String {
        let y = if op.y() { "t" } else { "b" };
        format!("slmaw{}{}  {}, {}, {}, {}", y,
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rd_alt()),
            Register::from_u32(op.rm()),
            Register::from_u32(op.rs()),
            Register::from_u32(op.rn_alt()),
        )
    }

    pub fn smulw_y(op: &MulBf) -> String {
        let y = if op.y() { "t" } else { "b" };
        format!("smulw{}{}  {}, {}, {}", y,
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rd_alt()),
            Register::from_u32(op.rm()),
            Register::from_u32(op.rs()),
        )
    }

    //
    // Multiply instructions
    //

    pub fn mul(op: &MulBf) -> String {
        format!("mul{}\t {}, {}, {}",
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rd_alt()),
            Register::from_u32(op.rm()),
            Register::from_u32(op.rs()),
        )
    }
    pub fn mla(op: &MulBf) -> String {
        format!("mul{}\t {}, {}, {}, {}",
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rd_alt()),
            Register::from_u32(op.rm()),
            Register::from_u32(op.rs()),
            Register::from_u32(op.rn_alt()),
        )
    }

    pub fn umull(op: &MulBf) -> String {
        format!("umull{}\t {}, {}, {}, {}",
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rd_lo()),
            Register::from_u32(op.rd_hi()),
            Register::from_u32(op.rm()),
            Register::from_u32(op.rs()),
        )
    }

    pub fn umlal(op: &MulBf) -> String {
        format!("umlal{}\t {}, {}, {}, {}",
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rd_lo()),
            Register::from_u32(op.rd_hi()),
            Register::from_u32(op.rm()),
            Register::from_u32(op.rs()),
        )
    }

    pub fn smlal(op: &MulBf) -> String {
        format!("smlal{}\t {}, {}, {}, {}",
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rd_lo()),
            Register::from_u32(op.rd_hi()),
            Register::from_u32(op.rm()),
            Register::from_u32(op.rs()),
        )
    }

    pub fn smull(op: &MulBf) -> String {
        format!("smull{}\t {}, {}, {}, {}",
            Cond::from_u32(op.cond()),
            Register::from_u32(op.rd_lo()),
            Register::from_u32(op.rd_hi()),
            Register::from_u32(op.rm()),
            Register::from_u32(op.rs()),
        )
    }

}


fn get_disas_str(op: &ArmInst, offset: u32) -> String {
    match op {

        // Data-processing
        ArmInst::DpRotImm(bf)   => dis::dp_rot_imm(bf),
        ArmInst::DpShiftImm(bf) => dis::dp_shift_imm(bf),
        ArmInst::DpShiftReg(bf) => dis::dp_shift_reg(bf),
        
        // Load/store
        ArmInst::LsMulti(bf)    => dis::ls_multi(bf),
        ArmInst::LsImm(bf)      => dis::ls_imm(bf),

        ArmInst::StrhLdrhImm(bf) => dis::ls_halfword_imm(bf),
        ArmInst::StrhLdrhReg(bf) => dis::ls_halfword_reg(bf),

        // Branching
        ArmInst::Branch(bf)     => dis::branch(bf, offset),
        ArmInst::BlxImm(bf)     => dis::blx_imm(bf, offset),
        ArmInst::BlxReg(bf)     => dis::blx_reg(bf),
        ArmInst::Bx(bf)         => dis::bx(bf),

        // Saturated add/sub
        ArmInst::Qadd(bf)       => dis::qadd(bf),
        ArmInst::Qsub(bf)       => dis::qsub(bf),
        ArmInst::QdAdd(bf)      => dis::qdadd(bf),
        ArmInst::QdSub(bf)      => dis::qdsub(bf),

        // Signed multiply (extended)
        ArmInst::SmlaXy(bf)     => dis::smla_xy(bf),
        ArmInst::SmlalXy(bf)    => dis::smlal_xy(bf),
        ArmInst::SmulXy(bf)     => dis::smul_xy(bf),
        ArmInst::SmlawY(bf)     => dis::smlaw_y(bf),
        ArmInst::SmulwY(bf)     => dis::smulw_y(bf),

        // Multiply instructions
        ArmInst::Mul(bf)        => dis::mul(bf),
        ArmInst::Mla(bf)        => dis::mla(bf),
        ArmInst::Umull(bf)      => dis::umull(bf),
        ArmInst::Umlal(bf)      => dis::umlal(bf),
        ArmInst::Smlal(bf)      => dis::smlal(bf),
        ArmInst::Smull(bf)      => dis::smull(bf),

        // Misc
        ArmInst::Swi(bf)        => dis::swi(bf),
        ArmInst::Bkpt(bf)       => dis::bkpt(bf),
        ArmInst::Swp(bf)        => dis::swp(bf),

        _ => format!("{:?}",op).to_string(),
    }
}


// Read some code into a Vec<u32>, then just iterate and decode/interpret
// each instruction.

fn main() {
    let mut buf = Vec::<u8>::new();
    //let mut file = std::fs::File::open("testsuite/arm_test.bin").unwrap();
    let mut file = std::fs::File::open("testsuite/arm_decode_test.bin").unwrap();
    file.read_to_end(&mut buf).unwrap();
    let databuf = make_u32_buf(&buf);

    run(&databuf);
}

fn run(databuf: &Vec<u32>) {
    //let mut offset = 0x8018u32;
        let mut offset = 0x0u32;
        for val in databuf.iter() {
            let instr = decode(*val);
            let disas_str = get_disas_str(&instr, offset);
            println!("{:04x}:\t {:08x}\t {}", offset, val, disas_str);
            offset += 4;
        }
}
