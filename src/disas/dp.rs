use armbf::newtype::*;
use armbf::traits::*;
use armbf::fields::*;

pub fn rot_imm_mov_cmp(op: &DpRotImmBf) -> String { 
    format!("{}{}\t {}, #{}", 
        Opcode::from_u32(op.opcd()), 
        Cond::from_u32(op.cond()), 
        Register::from_u32(op.rn()), 
        op.imm8()
    )
}
pub fn rot_imm_arith(op: &DpRotImmBf) -> String { 
    format!("{}{}\t {}, {}, #{}", 
        Opcode::from_u32(op.opcd()), 
        Cond::from_u32(op.cond()), 
        Register::from_u32(op.rd()), 
        Register::from_u32(op.rn()), 
        op.imm8(),
    )
}


pub fn shift_imm_arith(op: &DpShiftBf) -> String {
    let opcd =          Opcode::from_u32(op.opcd());
    let cond =          Cond::from_u32(op.cond());
    let rn =            Register::from_u32(op.rn());
    let rd =            Register::from_u32(op.rd());
    let rm =            Register::from_u32(op.rm());
    let shift_imm =     op.shift_imm();
    let shift_type =    ShifterType::from_u32(op.shift());
    if shift_imm == 0 {
        return format!("{}{}\t {}, {}, {}", opcd, cond, rd, rn, rm);
    }
    format!("{}{}\t {}, {}, {} {} #{}", opcd, cond, rd, rn, 
        rm, shift_type, shift_imm
    )
}
pub fn shift_imm_cmp(op: &DpShiftBf) -> String {
    let opcd =          Opcode::from_u32(op.opcd());
    let cond =          Cond::from_u32(op.cond());
    let rn =            Register::from_u32(op.rn());
    let rd =            Register::from_u32(op.rd());
    let rm =            Register::from_u32(op.rm());
    let shift_imm =     op.shift_imm();
    let shift_type =    ShifterType::from_u32(op.shift());
    if shift_imm == 0 {
        return format!("{}{}\t {}, {}", opcd, cond, rn, rm);
    }
    format!("{}{}\t {}, {}", opcd, cond, rn, 
        format!("{}, {} #{}", rm, shift_type, shift_imm)
    )
}
pub fn shift_imm_mov(op: &DpShiftBf) -> String {
    let opcd =          Opcode::from_u32(op.opcd());
    let cond =          Cond::from_u32(op.cond());
    let rn =            Register::from_u32(op.rn());
    let rd =            Register::from_u32(op.rd());
    let rm =            Register::from_u32(op.rm());
    let shift_imm =     op.shift_imm();
    let shift_type =    ShifterType::from_u32(op.shift());
    if shift_imm == 0 {
        return format!("{}{}\t {}, {}", opcd, cond, rd, rm);
    }
    match opcd {
        Opcode::Mov => format!("{}{}\t {}, {}, #{}", 
            shift_type, cond, rd, rm, shift_imm),
        Opcode::Mvn => format!("{}{}\t {}, {}, {} #{}", 
            opcd, cond, rd, rm, shift_type, shift_imm),
        _ => unreachable!(),
    }
}


pub fn shift_reg_arith(op: &DpShiftBf) -> String { 
    let opcd = Opcode::from_u32(op.opcd());
    let cond = Cond::from_u32(op.cond());
    let rn = Register::from_u32(op.rn());
    let rd = Register::from_u32(op.rd());
    let rm = Register::from_u32(op.rm());
    let rs = Register::from_u32(op.rs());
    let shift_type = ShifterType::from_u32(op.shift());
    format!("{}{}\t {}, {}, {} {} {}", opcd, cond, rd, rn, rm, shift_type, rs)

}
pub fn shift_reg_cmp(op: &DpShiftBf) -> String { 
    let opcd = Opcode::from_u32(op.opcd());
    let cond = Cond::from_u32(op.cond());
    let rn = Register::from_u32(op.rn());
    let rd = Register::from_u32(op.rd());
    let rm = Register::from_u32(op.rm());
    let rs = Register::from_u32(op.rs());
    let shift_type = ShifterType::from_u32(op.shift());
    format!("{}{}\t {}, {}", opcd, cond, rn, 
        format!("{}, {} {}", rm, shift_type, rs)
    )
}
pub fn shift_reg_mov(op: &DpShiftBf) -> String { 
    let opcd = Opcode::from_u32(op.opcd());
    let cond = Cond::from_u32(op.cond());
    let rn = Register::from_u32(op.rn());
    let rd = Register::from_u32(op.rd());
    let rm = Register::from_u32(op.rm());
    let rs = Register::from_u32(op.rs());
    let shift_type = ShifterType::from_u32(op.shift());
    match opcd {
        Opcode::Mov => format!("{}{}\t {}, {}, {}", 
            shift_type, cond, rd, rm, rs),
        Opcode::Mvn => format!("{}{}\t {}, {}, {} {}", 
            opcd, cond, rd, rm, shift_type, rs),
        _ => unreachable!(),
    }
}


