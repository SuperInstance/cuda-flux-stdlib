//! Agent utility bytecodes for FLUX VM.

use crate::StdlibFunction;

/// TRUST_CHECK: load trust score from memory region 0xFE00+r0. Returns score in r0.
pub const FN_TRUST_CHECK: &[u8] = &[
    0x30, 0x00, 0xFE, 0x00,  // LOAD r0, [0xFE00 + r0] — trust memory region
    0x59,                    // RET
];

/// ENERGY_CHECK: load energy register from system memory 0xFF00. Returns energy in r0.
pub const FN_ENERGY_CHECK: &[u8] = &[
    0x48, 0x00, 0xFF, 0x00,  // MOV r0, 0xFF00
    0x30, 0x00, 0x00,        // LOAD r0, [r0] — read energy register
    0x59,                    // RET
];

/// SHOULD_DELEGATE: compare energy (r0) to threshold (r1), check confidence (r2).
/// Returns r0 = 1 (delegate) or r0 = 0 (don't).
pub const FN_SHOULD_DELEGATE: &[u8] = &[
    0x48, 0x03, 0x00,        // MOV r3, 0       ; result = 0
    0x20, 0x00, 0x01,        // CMP r0, r1      ; energy vs threshold
    0x53, 0x06,              // JLT +6          ; if energy < threshold, delegate
    0x20, 0x02, 0x48, 0x32,  // CMP r2, 50     ; confidence vs 50%
    0x53, 0x03,              // JLT +3          ; if confidence < 50%, delegate
    0x50, 0x03,              // JMP +3 (don't delegate)
    0x48, 0x03, 0x01,        // MOV r3, 1       ; result = delegate
    0x48, 0x00, 0x03,        // MOV r0, r3
    0x59,                    // RET
];

/// REPORT_STATUS: prepare status for A2A TELL. Loads agent ID, state, and energy
/// into a message buffer at r0 (buffer addr). r1=agent_id, r2=state, r3=energy.
pub const FN_REPORT_STATUS: &[u8] = &[
    0x31, 0x00, 0x01,        // STORE [r0+0], r1  ; agent_id
    0x10, 0x00, 0x48, 0x01,  // ADDI r0, 1
    0x31, 0x00, 0x02,        // STORE [r0+1], r2  ; state
    0x10, 0x00, 0x48, 0x01,  // ADDI r0, 1
    0x31, 0x00, 0x03,        // STORE [r0+2], r3  ; energy
    0x10, 0x00, 0x48, 0x01,  // ADDI r0, 1
    0x48, 0x04, 0xA2, 0x01,  // MOV r4, 0xA201   ; TELL opcode
    0x31, 0x00, 0x04,        // STORE [r0+3], r4  ; message type
    0x59,                    // RET
];

pub fn register() -> Vec<StdlibFunction> {
    vec![
        StdlibFunction {
            name: "trust_check".into(),
            bytecodes: FN_TRUST_CHECK.to_vec(),
            param_count: 1,
            return_count: 1,
            stack_usage: 0,
            description: "Load trust score from trust memory region (0xFE00+id)".into(),
        },
        StdlibFunction {
            name: "energy_check".into(),
            bytecodes: FN_ENERGY_CHECK.to_vec(),
            param_count: 0,
            return_count: 1,
            stack_usage: 0,
            description: "Read energy register from system memory (0xFF00)".into(),
        },
        StdlibFunction {
            name: "should_delegate".into(),
            bytecodes: FN_SHOULD_DELEGATE.to_vec(),
            param_count: 3,
            return_count: 1,
            stack_usage: 0,
            description: "Decide whether to delegate based on energy threshold and confidence".into(),
        },
        StdlibFunction {
            name: "report_status".into(),
            bytecodes: FN_REPORT_STATUS.to_vec(),
            param_count: 4,
            return_count: 0,
            stack_usage: 0,
            description: "Prepare A2A TELL status message buffer".into(),
        },
    ]
}
