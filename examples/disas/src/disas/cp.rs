use armbf::newtype::*;
use armbf::traits::*;
use armbf::fields::*;

pub fn mrc(op: &CoprocBf) -> String {
    format!("mrc{}\t {}, {}, {}, {}, {}, {{{}}}",
        Cond::from_u32(op.cond()),
        CoprocNumber::from_u32(op.cp_num()),
        op.opcd1(),
        Register::from_u32(op.crd()),
        CoprocRegister::from_u32(op.crn()),
        CoprocRegister::from_u32(op.crm()),
        op.opcd2(),
    )
}

pub fn mcr(op: &CoprocBf) -> String {
    format!("mcr{}\t {}, {}, {}, {}, {}, {{{}}}",
        Cond::from_u32(op.cond()),
        CoprocNumber::from_u32(op.cp_num()),
        op.opcd1(),
        Register::from_u32(op.rd()),
        CoprocRegister::from_u32(op.crn()),
        CoprocRegister::from_u32(op.crm()),
        op.opcd2(),
    )
}


