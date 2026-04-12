//! Memory utility bytecodes for FLUX VM.

use crate::StdlibFunction;

/// MEMSET: fill r1 bytes at address r0 with value r2.
/// r0=addr, r1=count, r2=value → r3=counter
pub const FN_MEMSET: &[u8] = &[
    0x48, 0x03, 0x00,        // MOV r3, 0       ; i = 0
    // loop:
    0x20, 0x03, 0x01,        // CMP r3, r1      ; compare i, count
    0x55, 0x05,              // JEQ +5          ; if i == count, done
    0x31, 0x00, 0x02,        // STORE [r0+r3], r2
    0x10, 0x03, 0x48, 0x01,  // ADDI r3, 1
    0x50, 0xF6,              // JMP -10 (loop)
    0x59,                    // RET
];

/// MEMCPY: copy r2 bytes from r0 to r1.
/// r0=src, r1=dst, r2=count → r3=counter, r4=temp
pub const FN_MEMCPY: &[u8] = &[
    0x48, 0x03, 0x00,        // MOV r3, 0       ; i = 0
    // loop:
    0x20, 0x03, 0x02,        // CMP r3, r2
    0x55, 0x07,              // JEQ +7 (done)
    0x30, 0x04, 0x00,        // LOAD r4, [r0+r3]
    0x31, 0x01, 0x04,        // STORE [r1+r3], r4
    0x10, 0x03, 0x48, 0x01,  // ADDI r3, 1
    0x50, 0xF8,              // JMP -8 (loop)
    0x59,                    // RET
];

/// MEMCMP: compare r2 bytes at r0 vs r1. r0=0 if equal, r0=1 if not.
pub const FN_MEMCMP: &[u8] = &[
    0x48, 0x03, 0x00,        // MOV r3, 0       ; i = 0
    0x48, 0x04, 0x00,        // MOV r4, 0       ; result = equal
    // loop:
    0x20, 0x03, 0x02,        // CMP r3, r2
    0x55, 0x0A,              // JEQ +10 (done)
    0x30, 0x05, 0x00,        // LOAD r5, [r0+r3]
    0x30, 0x06, 0x01,        // LOAD r6, [r1+r3]
    0x20, 0x05, 0x06,        // CMP r5, r6
    0x55, 0x03,              // JEQ +3 (match, continue)
    0x48, 0x04, 0x01,        // MOV r4, 1       ; mismatch
    0x59,                    // RET (early exit)
    0x10, 0x03, 0x48, 0x01,  // ADDI r3, 1
    0x50, 0xF5,              // JMP -11 (loop)
    0x48, 0x00, 0x04,        // MOV r0, r4
    0x59,                    // RET
];

/// REVERSE: reverse r1 bytes at address r0 in place.
pub const FN_REVERSE: &[u8] = &[
    0x48, 0x02, 0x00,        // MOV r2, 0       ; left = 0
    0x11, 0x03, 0x01,        // SUB r3, r1      ; right = count
    0x11, 0x03, 0x48, 0x01,  // SUBI r3, 1      ; right = count - 1
    // loop:
    0x20, 0x02, 0x03,        // CMP r2, r3
    0x54, 0x0A,              // JGT +10 (done, left >= right)
    0x30, 0x04, 0x00,        // LOAD r4, [r0+r2] ; temp_l
    0x30, 0x05, 0x03,        // LOAD r5, [r0+r3] ; temp_r
    0x31, 0x00, 0x05,        // STORE [r0+r2], r5
    0x31, 0x03, 0x04,        // STORE [r0+r3], r4
    0x10, 0x02, 0x48, 0x01,  // ADDI r2, 1
    0x11, 0x03, 0x48, 0x01,  // SUBI r3, 1
    0x50, 0xF0,              // JMP -16 (loop)
    0x59,                    // RET
];

pub fn register() -> Vec<StdlibFunction> {
    vec![
        StdlibFunction {
            name: "memset".into(),
            bytecodes: FN_MEMSET.to_vec(),
            param_count: 3,
            return_count: 0,
            stack_usage: 0,
            description: "Fill memory region with a value".into(),
        },
        StdlibFunction {
            name: "memcpy".into(),
            bytecodes: FN_MEMCPY.to_vec(),
            param_count: 3,
            return_count: 0,
            stack_usage: 0,
            description: "Copy bytes from source to destination".into(),
        },
        StdlibFunction {
            name: "memcmp".into(),
            bytecodes: FN_MEMCMP.to_vec(),
            param_count: 3,
            return_count: 1,
            stack_usage: 0,
            description: "Compare two memory regions byte-by-byte".into(),
        },
        StdlibFunction {
            name: "reverse".into(),
            bytecodes: FN_REVERSE.to_vec(),
            param_count: 2,
            return_count: 0,
            stack_usage: 0,
            description: "Reverse bytes in a memory region in place".into(),
        },
    ]
}
