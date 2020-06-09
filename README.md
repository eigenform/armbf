# armdecode
Helper crate for decoding ARMv5 instructions. This is probably not very fast.

## Overview
```
├── benches/	- Benchmarks
├── bf/		- Some dependencies 
├── Cargo.toml
├── README.md
├── src/
└── testsuite/	- ARM code for running tests
```

## Usage
Presumably you plug this into some other program that needs to decode ARMv5
instructions. See the programs in [`src/bin/`](src/bin/) for an example.


