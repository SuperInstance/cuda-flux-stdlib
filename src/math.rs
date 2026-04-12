//! Math functions as FLUX bytecode constants.
//!
//! Opcodes (hex):
//!   NOP=0x00  HALT=0x01
//!   ADD=0x10  SUB=0x11  MUL=0x12  DIV=0x13  MOD=0x14  NEG=0x15
//!   AND=0x18  XOR=0x19  OR=0x1A  NOT=0x1B
//!   CMP=0x20
//!   LOAD=0x30  STORE=0x31
//!   PUSH=0x40  POP=0x41
//!   MOV=0x48   (format B: mov rd imm8)
//!   JMP=0x50  JZ=0x51  JNZ=0x52  JLT=0x53  JGT=0x54  JEQ=0x55  JNE=0x56
//!   CALL=0x58  RET=0x59  LOOP=0x5A

use crate::StdlibFunction;

/// ABS: negate if negative.
/// Params: r0=base_addr. Returns result in r0.
/// Bytecode:
///   LOAD r0, [r0]       ; load value from param
///   MOV  r1, 0           ; r1 = 0
///   CMP  r0, r1          ; compare value vs 0
///   JLT  +3              ; if negative, skip to NEG
///   RET
///   NEG  r0              ; negate
///   RET
pub const FN_ABS: &[u8] = &[
    0x30, 0x00,  // LOAD r0, [r0] — load value pointed to by r0
    0x48, 0x01, 0x00,  // MOV r1, 0
    0x20, 0x00, 0x01,  // CMP r0, r1
    0x53, 0x03,        // JLT +3
    0x59,              // RET
    0x15, 0x00,        // NEG r0
    0x59,              // RET
];

/// MAX: compare r0 and r1, put larger in r0.
///   CMP r0, r1
///   JGE +2
///   MOV r0, r1  (via XOR trick: r0 = r0 ^ r1 ^ r0... actually just MOV)
///   RET
pub const FN_MAX: &[u8] = &[
    0x20, 0x00, 0x01,  // CMP r0, r1
    0x54, 0x02,        // JGT +2 (if r0 > r1, skip swap)
    // swap: temp via stack
    0x40, 0x00,        // PUSH r0
    0x48, 0x00, 0x01,  // MOV r0, r1
    0x41, 0x01,        // POP r1
    0x59,              // RET
];

/// MIN: compare r0 and r1, put smaller in r0.
pub const FN_MIN: &[u8] = &[
    0x20, 0x00, 0x01,  // CMP r0, r1
    0x53, 0x02,        // JLT +2 (if r0 < r1, skip swap)
    0x40, 0x00,        // PUSH r0
    0x48, 0x00, 0x01,  // MOV r0, r1
    0x41, 0x01,        // POP r1
    0x59,              // RET
];

/// CLAMP: clamp r0 to [r1, r2].
///   CMP r0, r1  → if < min, set r0 = r1
///   CMP r0, r2  → if > max, set r0 = r2
pub const FN_CLAMP: &[u8] = &[
    0x20, 0x00, 0x01,  // CMP r0, r1
    0x53, 0x02,        // JLT +2
    0x50, 0x06,        // JMP +6 (skip min clamp)
    0x48, 0x00, 0x01,  // MOV r0, r1
    0x20, 0x00, 0x02,  // CMP r0, r2
    0x54, 0x02,        // JGT +2
    0x50, 0x02,        // JMP +2 (skip max clamp)
    0x48, 0x00, 0x02,  // MOV r0, r2
    0x59,              // RET
];

/// SWAP: XOR swap r0 and r1 (3 XOR operations).
pub const FN_SWAP: &[u8] = &[
    0x19, 0x00, 0x01,  // XOR r0, r1  ; r0 = r0 ^ r1
    0x19, 0x01, 0x00,  // XOR r1, r0  ; r1 = r1 ^ (r0^r1) = orig r0
    0x19, 0x00, 0x01,  // XOR r0, r1  ; r0 = (r0^r1) ^ orig r0 = orig r1
    0x59,              // RET
];

/// POWER: r0^r1 → r0. Uses r2 as counter, r3 as accumulator.
pub const FN_POWER: &[u8] = &[
    0x48, 0x03, 0x01,  // MOV r3, 1       ; acc = 1
    0x48, 0x02, 0x00,  // MOV r2, 0       ; i = 0
    0x20, 0x02, 0x01,  // CMP r2, r1      ; compare i, exponent
    0x55, 0x04,        // JEQ +4          ; if i == exp, done
    0x12, 0x03, 0x00,  // MUL r3, r0      ; acc *= base
    0x10, 0x02, 0x48, 0x01,  // ADDI r2, 1
    0x50, 0xF6,        // JMP -10 (back to CMP)
    0x48, 0x00, 0x03,  // MOV r0, r3      ; result = acc
    0x59,              // RET
];

/// GCD: Euclidean algorithm. r0, r1 → r0 = gcd(r0, r1).
pub const FN_GCD: &[u8] = &[
    // loop:
    0x20, 0x01, 0x48, 0x00,  // CMP r1, 0
    0x51, 0x06,              // JZ +6 (done)
    0x48, 0x02, 0x00,        // MOV r2, r0      ; temp = a
    0x14, 0x00, 0x01,        // MOD r0, r1      ; a = a % b
    0x48, 0x01, 0x02,        // MOV r1, r2      ; b = temp
    0x50, 0xF5,              // JMP -11 (loop)
    0x59,                    // RET
];

pub fn register() -> Vec<StdlibFunction> {
    vec![
        StdlibFunction {
            name: "abs".into(),
            bytecodes: FN_ABS.to_vec(),
            param_count: 1,
            return_count: 1,
            stack_usage: 0,
            description: "Absolute value: negate if negative".into(),
        },
        StdlibFunction {
            name: "max".into(),
            bytecodes: FN_MAX.to_vec(),
            param_count: 2,
            return_count: 1,
            stack_usage: 2,
            description: "Maximum of two values".into(),
        },
        StdlibFunction {
            name: "min".into(),
            bytecodes: FN_MIN.to_vec(),
            param_count: 2,
            return_count: 1,
            stack_usage: 2,
            description: "Minimum of two values".into(),
        },
        StdlibFunction {
            name: "clamp".into(),
            bytecodes: FN_CLAMP.to_vec(),
            param_count: 3,
            return_count: 1,
            stack_usage: 0,
            description: "Clamp value to [min, max] range".into(),
        },
        StdlibFunction {
            name: "swap".into(),
            bytecodes: FN_SWAP.to_vec(),
            param_count: 2,
            return_count: 2,
            stack_usage: 0,
            description: "XOR swap two registers".into(),
        },
        StdlibFunction {
            name: "power".into(),
            bytecodes: FN_POWER.to_vec(),
            param_count: 2,
            return_count: 1,
            stack_usage: 0,
            description: "Exponentiation: base^exp via loop multiply".into(),
        },
        StdlibFunction {
            name: "gcd".into(),
            bytecodes: FN_GCD.to_vec(),
            param_count: 2,
            return_count: 1,
            stack_usage: 0,
            description: "Greatest common divisor via Euclidean algorithm".into(),
        },
    ]
}
