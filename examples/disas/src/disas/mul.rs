use armbf::newtype::*;
use armbf::traits::*;
use armbf::fields::*;


pub fn mul(op: &MulBf) -> String {
    format!("mul{}\t {}, {}, {}",
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
    )
}
pub fn mla(op: &MulBf) -> String {
    format!("mul{}\t {}, {}, {}, {}",
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
        Register::from_u32(op.rn()),
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


pub fn smla_xy(op: &MulBf) -> String {
    let xy = match (op.x(), op.y()) {
        (false, false) => "bb", (false, true) => "bt",
        (true, false) => "tb", (true, true) => "tt",
    };
    format!("slma{}{}\t {}, {}, {}, {}", xy,
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
        Register::from_u32(op.rn()),
    )
}

pub fn smlal_xy(op: &MulBf) -> String {
    let xy = match (op.x(), op.y()) {
        (false, false) => "bb", (false, true) => "bt",
        (true, false) => "tb", (true, true) => "tt",
    };
    format!("slmal{}{} {}, {}, {}, {}", xy,
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rn()),
        Register::from_u32(op.rd()),
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
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
    )
}

pub fn smlaw_y(op: &MulBf) -> String {
    let y = if op.y() { "t" } else { "b" };
    format!("slmaw{}{}  {}, {}, {}, {}", y,
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
        Register::from_u32(op.rn()),
    )
}

pub fn smulw_y(op: &MulBf) -> String {
    let y = if op.y() { "t" } else { "b" };
    format!("smulw{}{}  {}, {}, {}", y,
        Cond::from_u32(op.cond()),
        Register::from_u32(op.rd()),
        Register::from_u32(op.rm()),
        Register::from_u32(op.rs()),
    )
}


