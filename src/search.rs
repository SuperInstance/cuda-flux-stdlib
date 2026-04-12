//! Search algorithms as FLUX bytecode.

use crate::StdlibFunction;

/// LINEAR_SEARCH: find value r1 in array at r0 (length r2). Returns index in r0 or 0xFF.
pub const FN_LINEAR_SEARCH: &[u8] = &[
    0x48, 0x03, 0x00,        // MOV r3, 0       ; i = 0
    // loop:
    0x20, 0x03, 0x02,        // CMP r3, r2      ; i < len?
    0x54, 0x07,              // JGT +7 (not found)
    0x30, 0x04, 0x00,        // LOAD r4, [r0+r3]
    0x20, 0x04, 0x01,        // CMP r4, r1
    0x55, 0x03,              // JEQ +3 (found!)
    0x10, 0x03, 0x48, 0x01,  // ADDI r3, 1
    0x50, 0xF6,              // JMP -10 (loop)
    0x48, 0x00, 0xFF,        // MOV r0, 0xFF (not found)
    0x59,                    // RET
    // found:
    0x48, 0x00, 0x03,        // MOV r0, r3
    0x59,                    // RET
];

/// BINARY_SEARCH: find r1 in sorted array at r0 (length r2). Returns index in r0 or 0xFF.
pub const FN_BINARY_SEARCH: &[u8] = &[
    0x48, 0x03, 0x00,        // MOV r3, 0       ; lo = 0
    0x11, 0x04, 0x02,        // SUB r4, r2      ; hi = len
    0x11, 0x04, 0x48, 0x01,  // SUBI r4, 1      ; hi = len - 1
    // loop:
    0x20, 0x03, 0x04,        // CMP r3, r4
    0x54, 0x11,              // JGT +17 (not found, lo > hi)
    // mid = (lo + hi) / 2
    0x48, 0x05, 0x03,        // MOV r5, r3
    0x10, 0x05, 0x04,        // ADD r5, r4
    0x13, 0x05, 0x48, 0x02,  // DIVI r5, 2
    0x30, 0x06, 0x00,        // LOAD r6, [r0+r5]
    0x20, 0x06, 0x01,        // CMP r6, r1
    0x55, 0x08,              // JEQ +8 (found!)
    0x53, 0x04,              // JLT +4 (arr[mid] < target → lo = mid+1)
    // arr[mid] > target → hi = mid - 1
    0x11, 0x04, 0x05,        // SUB r4, r5
    0x11, 0x04, 0x48, 0x01,  // SUBI r4, 1
    0x50, 0xF0,              // JMP -16 (loop)
    // lo = mid + 1
    0x10, 0x03, 0x05,        // ADD r3, r5
    0x10, 0x03, 0x48, 0x01,  // ADDI r3, 1
    0x50, 0xF5,              // JMP -11 (loop)
    // not found:
    0x48, 0x00, 0xFF,        // MOV r0, 0xFF
    0x59,                    // RET
    // found:
    0x48, 0x00, 0x05,        // MOV r0, r5
    0x59,                    // RET
];

pub fn register() -> Vec<StdlibFunction> {
    vec![
        StdlibFunction {
            name: "linear_search".into(),
            bytecodes: FN_LINEAR_SEARCH.to_vec(),
            param_count: 3,
            return_count: 1,
            stack_usage: 0,
            description: "Linear search: find value in array, return index or 0xFF".into(),
        },
        StdlibFunction {
            name: "binary_search".into(),
            bytecodes: FN_BINARY_SEARCH.to_vec(),
            param_count: 3,
            return_count: 1,
            stack_usage: 0,
            description: "Binary search: find value in sorted array, return index or 0xFF".into(),
        },
    ]
}
