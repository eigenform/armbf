use armbf::newtype::*;
use armbf::traits::*;
use armbf::fields::*;

pub fn svc(op: &SwiBf) -> String { 
    format!("svc\t 0x{:08x}", op.imm24())
}

pub fn bkpt(op: &BkptBf) -> String { 
    format!("bkpt\t 0x{:04x}", ((op.imm12_hi() << 4) | op.imm4()) as u16)
}

pub fn mrs(op: &StatusBf) -> String {
    let sr_name = if op.r() { "SPSR" } else { "CPSR" };
    format!("mrs{}\t {}, {}", Cond::from_u32(op.cond()), 
        Register::from_u32(op.rd()), sr_name
    )
}

pub fn msr_imm(op: &StatusBf) -> String {
    let sr_name = if op.r() { "SPSR" } else { "CPSR" };

    let mut fields_str = std::string::String::new();
    for idx in (0..4).rev() {
        if (op.field_mask() & (1 << idx)) != 0 {
            let bit = match idx {
                0 => "c",
                1 => "x",
                2 => "s",
                3 => "f",
                _ => unreachable!(),
            };
            fields_str.push_str(bit);
        }
    }
    format!("msr{}\t {}_{}, #{}", Cond::from_u32(op.cond()), sr_name,
        fields_str, op.imm8(),
    )
}

pub fn msr_reg(op: &StatusBf) -> String {
    let sr_name = if op.r() { "SPSR" } else { "CPSR" };

    let mut fields_str = std::string::String::new();
    for idx in (0..4).rev() {
        if (op.field_mask() & (1 << idx)) != 0 {
            let bit = match idx {
                0 => "c",
                1 => "x",
                2 => "s",
                3 => "f",
                _ => unreachable!(),
            };
            fields_str.push_str(bit);
        }
    }

    format!("msr{}\t {}_{}, {}",
        Cond::from_u32(op.cond()), 
        sr_name,
        fields_str,
        Register::from_u32(op.rm()),
    )
}


pub fn qadd(op: &SatBf) -> String { format!("qadd{}\t {}, {}, {}",
        Cond::from_u32(op.cond()), 
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()), 
        Register::from_u32(op.rn()),
    )
}
pub fn qdadd(op: &SatBf) -> String { format!("qdadd{}\t {}, {}, {}",
        Cond::from_u32(op.cond()), 
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()), 
        Register::from_u32(op.rn()),
    )
}
pub fn qsub(op: &SatBf) -> String { format!("qsub{}\t {}, {}, {}",
        Cond::from_u32(op.cond()), 
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()), 
        Register::from_u32(op.rn()),
    )
}
pub fn qdsub(op: &SatBf) -> String { format!("qdsub{}\t {}, {}, {}",
        Cond::from_u32(op.cond()), 
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()), 
        Register::from_u32(op.rn()),
    )
}

pub fn clz(op: &ClzBf) -> String { format!("clz{}\t {}, {}",
        Cond::from_u32(op.cond()), Register::from_u32(op.rd()),
        Register::from_u32(op.rm()),
    )
}


