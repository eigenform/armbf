# armdecode
Helper crate for decoding ARMv5 instructions. This is probably not very fast.

## Usage
Presumably you plug this into some other program that needs to decode ARMv5
instructions. For instance:

```rust
use armdecode;
...

fn interpret_arm(op: u32) {
    let instr: ArmInst = armdecode::decode(op);
    ...

    // Branch to an implementation of this instruction

    ...
} 

fn disas_arm(op: u32) {
    let instr: ArmInst = armdecode::decode(op);
    ...

    // Print a human-readable representation of this instruction

    ...
}

```
