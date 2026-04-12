//! Sorting algorithms as FLUX bytecode.

use crate::StdlibFunction;

/// BUBBLE_SORT: sort r1 elements in array at r0. Uses r2=outer i, r3=inner j, r4=temp.
pub const FN_BUBBLE_SORT: &[u8] = &[
    // outer: i from 0 to n-1
    0x48, 0x02, 0x00,        // MOV r2, 0       ; i = 0
    // outer_loop:
    0x11, 0x05, 0x01,        // SUB r5, r1
    0x11, 0x05, 0x48, 0x01,  // SUBI r5, 1      ; r5 = n-1
    0x20, 0x02, 0x05,        // CMP r2, r5
    0x54, 0x14,              // JGT +20 (done)
    // inner: j from 0 to n-i-2
    0x48, 0x03, 0x00,        // MOV r3, 0       ; j = 0
    // inner_loop:
    0x11, 0x05, 0x01,        // SUB r5, r1      ; r5 = n
    0x11, 0x05, 0x02,        // SUB r5, r2      ; r5 = n - i
    0x11, 0x05, 0x48, 0x01,  // SUBI r5, 1      ; r5 = n - i - 1
    0x11, 0x05, 0x48, 0x01,  // SUBI r5, 1      ; r5 = n - i - 2
    0x20, 0x03, 0x05,        // CMP r3, r5
    0x54, 0x0A,              // JGT +10 (inner done)
    // compare arr[j] vs arr[j+1]
    0x30, 0x04, 0x00,        // LOAD r4, [r0+r3]
    0x48, 0x06, 0x03,        // MOV r6, r3
    0x10, 0x06, 0x48, 0x01,  // ADDI r6, 1
    0x30, 0x07, 0x06,        // LOAD r7, [r0+r6]
    0x20, 0x04, 0x07,        // CMP r4, r7
    0x54, 0x02,              // JGT +2 (no swap needed)
    // swap arr[j] and arr[j+1]
    0x31, 0x00, 0x07,        // STORE [r0+r3], r7
    0x31, 0x06, 0x04,        // STORE [r0+r6], r4
    0x10, 0x03, 0x48, 0x01,  // ADDI r3, 1
    0x50, 0xF0,              // JMP -16 (inner_loop)
    0x10, 0x02, 0x48, 0x01,  // ADDI r2, 1
    0x50, 0xF0,              // JMP -16 (outer_loop)
    0x59,                    // RET
];

/// SELECTION_SORT: sort r1 elements in array at r0.
/// r2=outer i, r3=inner j, r4=min_idx, r5=temp.
pub const FN_SELECTION_SORT: &[u8] = &[
    0x48, 0x02, 0x00,        // MOV r2, 0       ; i = 0
    // outer_loop:
    0x11, 0x06, 0x01,        // SUB r6, r1
    0x11, 0x06, 0x48, 0x01,  // SUBI r6, 1      ; r6 = n-1
    0x20, 0x02, 0x06,        // CMP r2, r6
    0x54, 0x17,              // JGT +23 (done)
    0x48, 0x04, 0x02,        // MOV r4, r2      ; min_idx = i
    0x10, 0x03, 0x02,        // ADD r3, r2      ; j = i
    0x10, 0x03, 0x48, 0x01,  // ADDI r3, 1      ; j = i + 1
    // inner_loop:
    0x20, 0x03, 0x01,        // CMP r3, r1
    0x54, 0x0C,              // JGT +12 (inner done)
    0x30, 0x05, 0x04,        // LOAD r5, [r0+r4] ; min_val
    0x30, 0x06, 0x03,        // LOAD r6, [r0+r3] ; arr[j]
    0x20, 0x06, 0x05,        // CMP r6, r5
    0x54, 0x03,              // JGT +3 (no update)
    0x48, 0x04, 0x03,        // MOV r4, r3      ; min_idx = j
    0x10, 0x03, 0x48, 0x01,  // ADDI r3, 1
    0x50, 0xF3,              // JMP -13 (inner_loop)
    // swap arr[i] and arr[min_idx]
    0x30, 0x05, 0x02,        // LOAD r5, [r0+r2]
    0x30, 0x06, 0x04,        // LOAD r6, [r0+r4]
    0x31, 0x02, 0x06,        // STORE [r0+r2], r6
    0x31, 0x04, 0x05,        // STORE [r0+r4], r5
    0x10, 0x02, 0x48, 0x01,  // ADDI r2, 1
    0x50, 0xF0,              // JMP -16 (outer_loop)
    0x59,                    // RET
];

pub fn register() -> Vec<StdlibFunction> {
    vec![
        StdlibFunction {
            name: "bubble_sort".into(),
            bytecodes: FN_BUBBLE_SORT.to_vec(),
            param_count: 2,
            return_count: 0,
            stack_usage: 0,
            description: "Bubble sort array in ascending order".into(),
        },
        StdlibFunction {
            name: "selection_sort".into(),
            bytecodes: FN_SELECTION_SORT.to_vec(),
            param_count: 2,
            return_count: 0,
            stack_usage: 0,
            description: "Selection sort array in ascending order".into(),
        },
    ]
}
