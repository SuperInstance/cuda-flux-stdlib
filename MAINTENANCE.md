# cuda-flux-stdlib — Maintenance Notes

## Purpose
Standard library of bytecode subroutines for FLUX VM programs. These are pre-compiled bytecode patterns that any FLUX program can call via OP_CALL.

## Architecture
- Each function is a Vec&lt;u8&gt; of real FLUX bytecodes
- Functions use specific register conventions (documented per-function)
- Max 128 bytes per function (fits in a single memory page)
- 19 functions across 5 modules: math, memory, search, sorting, agent

## Call Convention
1. Set up parameters in designated registers
2. OP_CALL with subroutine address
3. Function executes, result in designated register
4. OP_RET returns to caller

## Key Insight: Bytecode as API
These functions are the STANDARD LIBRARY — they define the idiomatic way to do common operations in FLUX bytecode. If you need to sort an array, you don't write your own bubble sort — you call stdlib_bubble_sort. This creates a shared vocabulary across all fleet vessels.

## Adding Functions
1. Write bytecodes using real FLUX opcode values (see flux-runtime-c MAINTENANCE.md)
2. Keep under 128 bytes
3. Document register convention (which registers are inputs/outputs)
4. Add test that verifies bytecodes contain expected opcodes
5. Update README function table

## Related Crates
- flux-runtime-c: the VM that executes these bytecodes
- cuda-instruction-set: opcode definitions
- flux-asm: assembler (could compile these from text in the future)
