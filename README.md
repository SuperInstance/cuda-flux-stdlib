# cuda-flux-stdlib

> *The Deeper Connection.*

Standard library for **FLUX VM** programs — a collection of useful bytecode subroutine patterns defined as Rust constants.

## Overview

FLUX VM is a register-based virtual machine with a compact instruction set. This crate provides pre-built bytecode sequences for common operations, ready to be loaded into any FLUX VM instance.

## FLUX Opcode Reference

| Hex | Mnemonic | Format | Description |
|-----|----------|--------|-------------|
| `0x00` | NOP | — | No operation |
| `0x01` | HALT | — | Halt execution |
| `0x10` | ADD | A: rd, rs | Add rs to rd |
| `0x11` | SUB | A: rd, rs | Subtract rs from rd |
| `0x12` | MUL | A: rd, rs | Multiply rd by rs |
| `0x13` | DIV | A: rd, rs | Divide rd by rs |
| `0x14` | MOD | A: rd, rs | Modulo rd by rs |
| `0x15` | NEG | A: rd | Negate rd |
| `0x18` | AND | A: rd, rs | Bitwise AND |
| `0x19` | XOR | A: rd, rs | Bitwise XOR |
| `0x1A` | OR | A: rd, rs | Bitwise OR |
| `0x1B` | NOT | A: rd | Bitwise NOT |
| `0x20` | CMP | A: rd, rs | Compare rd and rs |
| `0x30` | LOAD | A: rd, rs | Load from memory |
| `0x31` | STORE | A: rd, rs | Store to memory |
| `0x40` | PUSH | A: rd | Push rd to stack |
| `0x41` | POP | A: rd | Pop stack into rd |
| `0x48` | MOV | B: rd, imm8 | Move immediate |
| `0x50` | JMP | B: offset | Unconditional jump |
| `0x51` | JZ | B: offset | Jump if zero |
| `0x52` | JNZ | B: offset | Jump if not zero |
| `0x53` | JLT | B: offset | Jump if less than |
| `0x54` | JGT | B: offset | Jump if greater than |
| `0x55` | JEQ | B: offset | Jump if equal |
| `0x56` | JNE | B: offset | Jump if not equal |
| `0x58` | CALL | B: addr | Call subroutine |
| `0x59` | RET | — | Return from subroutine |
| `0x5A` | LOOP | B: count | Loop with counter |

## Function Reference

### Math (`src/math.rs`)

| Name | Bytes | Params | Returns | Description |
|------|-------|--------|---------|-------------|
| `abs` | `30 00 48 01 00 20 00 01 53 03 59 15 00 59` | 1 | 1 | Absolute value: negate if negative |
| `max` | `20 00 01 54 02 40 00 48 00 01 41 01 59` | 2 | 1 | Maximum of two values |
| `min` | `20 00 01 53 02 40 00 48 00 01 41 01 59` | 2 | 1 | Minimum of two values |
| `clamp` | `20 00 01 53 02 50 06 48 00 01 20 00 02 54 02 50 02 48 00 02 59` | 3 | 1 | Clamp value to [min, max] |
| `swap` | `19 00 01 19 01 00 19 00 01 59` | 2 | 2 | XOR swap two registers |
| `power` | `48 03 01 48 02 00 20 02 01 55 04 12 03 00 10 02 48 01 50 F6 48 00 03 59` | 2 | 1 | Exponentiation via loop multiply |
| `gcd` | `20 01 48 00 51 06 48 02 00 14 00 01 48 01 02 50 F5 59` | 2 | 1 | GCD via Euclidean algorithm |

### Memory (`src/memory.rs`)

| Name | Params | Returns | Description |
|------|--------|---------|-------------|
| `memset` | 3 (addr, count, value) | 0 | Fill memory region with value |
| `memcpy` | 3 (src, dst, count) | 0 | Copy bytes between regions |
| `memcmp` | 3 (a, b, count) | 1 | Compare regions byte-by-byte |
| `reverse` | 2 (addr, count) | 0 | Reverse bytes in place |

### Search (`src/search.rs`)

| Name | Params | Returns | Description |
|------|--------|---------|-------------|
| `linear_search` | 3 (arr, val, len) | 1 (index or 0xFF) | Linear scan for value |
| `binary_search` | 3 (arr, val, len) | 1 (index or 0xFF) | Binary search on sorted array |

### Sorting (`src/sorting.rs`)

| Name | Params | Returns | Description |
|------|--------|---------|-------------|
| `bubble_sort` | 2 (arr, len) | 0 | Bubble sort ascending |
| `selection_sort` | 2 (arr, len) | 0 | Selection sort ascending |

### Agent (`src/agent.rs`)

| Name | Params | Returns | Description |
|------|--------|---------|-------------|
| `trust_check` | 1 (agent_id) | 1 (trust score) | Read from trust memory region (0xFE00+) |
| `energy_check` | 0 | 1 (energy) | Read energy register (0xFF00) |
| `should_delegate` | 3 (energy, threshold, confidence) | 1 (bool) | Decision: delegate or not |
| `report_status` | 4 (buf, id, state, energy) | 0 | Prepare A2A TELL message buffer |

## Usage

```rust
use cuda_flux_stdlib::Stdlib;

let lib = Stdlib::new();

if let Some(abs_fn) = lib.get("abs") {
    println!("ABS: {} bytes, {} params", abs_fn.bytecodes.len(), abs_fn.param_count);
    // Load abs_fn.bytecodes into your FLUX VM
}

for f in lib.all() {
    println!("{}: {:?}", f.name, f.bytecodes);
}
```

## Testing

```bash
cargo test
```

## License

MIT
