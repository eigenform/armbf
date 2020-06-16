use armbf::newtype::*;
use armbf::traits::*;
use armbf::fields::*;

pub fn ls_reg(op: &LsShiftBf, name: &'static str) -> String { 
    let fmt = if op.shift_imm() == 0 {
        match (op.p(), op.w()) {
            (true, false) => format!("[{}, {}]",
                Register::from_u32(op.rn()), Register::from_u32(op.rm())),
            (true, true) => format!("[{}, {}]!",
                Register::from_u32(op.rn()), Register::from_u32(op.rm())),
            (false, false) => format!("[{}], {}",
                Register::from_u32(op.rn()), Register::from_u32(op.rm())),
            _ => panic!(),
        }
    }
    else { 
        match (op.p(), op.w()) {
            (true, false) => format!("[{}, {}, {} #{}]",
                Register::from_u32(op.rn()), Register::from_u32(op.rm()),
                ShifterType::from_u32(op.shift()), op.shift_imm()
                ),
            (true, true) => format!("[{}, {}, {} #{}]!",
                Register::from_u32(op.rn()), Register::from_u32(op.rm()),
                ShifterType::from_u32(op.shift()), op.shift_imm()
                ),
            (false, false) => format!("[{}], {}, {} #{}",
                Register::from_u32(op.rn()), Register::from_u32(op.rm()),
                ShifterType::from_u32(op.shift()), op.shift_imm()
                ),
            _ => panic!(),
        }
    };

    format!("{}\t {}, {}", name, Register::from_u32(op.rd()), fmt)
}
pub fn ldrb_reg(op: &LsShiftBf) -> String { ls_reg(op, "ldrb") }
pub fn strb_reg(op: &LsShiftBf) -> String { ls_reg(op, "strb") }
pub fn ldr_reg(op: &LsShiftBf) -> String { ls_reg(op, "ldr") }
pub fn str_reg(op: &LsShiftBf) -> String { ls_reg(op, "str") }


pub fn ls_imm(op: &LsImmBf, name: &'static str) -> String {
    let fmt = match (op.p(), op.w()) {
        (true, false) => format!("[{}, #{}]", 
            Register::from_u32(op.rn()), op.imm12()),
        (true, true) => format!("[{}, #{}]!",
            Register::from_u32(op.rn()), op.imm12()),
        (false, false) => format!("[{}], #{}",
            Register::from_u32(op.rn()), op.imm12()),
        _ => panic!(),
    };
    format!("{}\t {}, {}", name, Register::from_u32(op.rd()), fmt)
}
pub fn ldrb_imm(op: &LsImmBf) -> String { ls_imm(op, "ldrb") }
pub fn strb_imm(op: &LsImmBf) -> String { ls_imm(op, "strb") }
pub fn ldr_imm(op: &LsImmBf) -> String { ls_imm(op, "ldr") }
pub fn str_imm(op: &LsImmBf) -> String { ls_imm(op, "str") }


pub fn ls_multi(op: &LsMultiBf, name: &'static str) -> String {
    let mut reglist_str = std::string::String::new();
    let rn = Register::from_u32(op.rn());
    let wb = if op.w() { "!" } else { "" };

    for idx in 0..16 {
        if (op.reglist() & (1 << idx)) != 0 {
            reglist_str.push_str( format!("{}, ", 
                    Register::from_u32(idx)).as_ref()
            );
        }
    }
    reglist_str.truncate(reglist_str.len() - 2);
    format!("{} {}{}, {{{}}}", name, rn, wb, reglist_str)
}
pub fn ldmib(op: &LsMultiBf) -> String { ls_multi(op, "ldmib") }
pub fn ldmia(op: &LsMultiBf) -> String { ls_multi(op, "ldmia") }
pub fn ldmdb(op: &LsMultiBf) -> String { ls_multi(op, "ldmdb") }
pub fn ldmda(op: &LsMultiBf) -> String { ls_multi(op, "ldmda") }
pub fn stmib(op: &LsMultiBf) -> String { ls_multi(op, "stmib") }
pub fn stmia(op: &LsMultiBf) -> String { ls_multi(op, "stmia") }
pub fn stmdb(op: &LsMultiBf) -> String { ls_multi(op, "stmdb") }
pub fn stmda(op: &LsMultiBf) -> String { ls_multi(op, "stmda") }



pub fn ls_misc_imm(op: &LsMiscBf, name: &'static str) -> String {
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
pub fn ldrh_imm(op: &LsMiscBf) -> String { ls_misc_imm(op, "ldrh") }
pub fn strh_imm(op: &LsMiscBf) -> String { ls_misc_imm(op, "strh") }
pub fn strd_imm(op: &LsMiscBf) -> String { ls_misc_imm(op, "strd") }
pub fn ldrd_imm(op: &LsMiscBf) -> String { ls_misc_imm(op, "ldrd") }
pub fn ldrsh_imm(op: &LsMiscBf) -> String { ls_misc_imm(op, "ldrsh") }
pub fn ldrsb_imm(op: &LsMiscBf) -> String { ls_misc_imm(op, "ldrsb") }


pub fn ls_misc_reg(op: &LsMiscBf, name: &'static str) -> String {
    format!("{}{}\t {}, [{}, {}]", name,
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd()),
        Register::from_u32(op.rn()),
        Register::from_u32(op.rm()),
    )
}
pub fn ldrh_reg(op: &LsMiscBf) -> String { ls_misc_reg(op, "ldrh") }
pub fn strh_reg(op: &LsMiscBf) -> String { ls_misc_reg(op, "strh") }
pub fn strd_reg(op: &LsMiscBf) -> String { ls_misc_reg(op, "strd") }
pub fn ldrd_reg(op: &LsMiscBf) -> String { ls_misc_reg(op, "ldrd") }
pub fn ldrsh_reg(op: &LsMiscBf) -> String { ls_misc_reg(op, "ldrsh") }
pub fn ldrsb_reg(op: &LsMiscBf) -> String { ls_misc_reg(op, "ldrsb") }


pub fn swp_generic(op: &SwpBf, name: &'static str) -> String { 
    format!("{}\t {}, {}, [{}]", name, Register::from_u32(op.rd()), 
        Register::from_u32(op.rm()), Register::from_u32(op.rn()),
    )
}
pub fn swp(op: &SwpBf) -> String { swp_generic(op, "swp") }
pub fn swpb(op: &SwpBf) -> String { swp_generic(op, "swpb") }



