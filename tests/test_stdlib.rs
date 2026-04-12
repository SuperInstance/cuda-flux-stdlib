use cuda_flux_stdlib::*;

#[test]
fn all_functions_have_valid_bytecodes() {
    let lib = Stdlib::new();
    for f in lib.all() {
        assert!(!f.bytecodes.is_empty(), "{} has empty bytecodes", f.name);
        // Must end with RET (0x59) or HALT (0x01)
        let last = *f.bytecodes.last().unwrap();
        assert!(last == 0x59 || last == 0x01, "{} doesn't end with RET/HALT", f.name);
    }
}

#[test]
fn param_counts_are_reasonable() {
    let lib = Stdlib::new();
    for f in lib.all() {
        assert!(f.param_count <= 4, "{}: param_count {} > 4", f.name, f.param_count);
        assert!(f.return_count <= 2, "{}: return_count {} > 2", f.name, f.return_count);
        assert!(f.stack_usage <= 4, "{}: stack_usage {} > 4", f.name, f.stack_usage);
    }
}

#[test]
fn no_function_exceeds_128_bytes() {
    let lib = Stdlib::new();
    for f in lib.all() {
        assert!(f.bytecodes.len() <= 128,
            "{}: {} bytes exceeds 128 limit", f.name, f.bytecodes.len());
    }
}

#[test]
fn abs_handles_negative() {
    let abs = lib().get("abs").unwrap();
    // Bytecode should contain NEG opcode (0x15)
    assert!(abs.bytecodes.contains(&0x15), "abs missing NEG opcode");
    // Should contain JLT (0x53) for negative check
    assert!(abs.bytecodes.contains(&0x53), "abs missing JLT opcode");
}

#[test]
fn clamp_bounds_correct() {
    let clamp = lib().get("clamp").unwrap();
    // Should have two CMP instructions (min and max checks)
    let cmp_count = clamp.bytecodes.iter().filter(|&&b| b == 0x20).count();
    assert!(cmp_count >= 2, "clamp should have at least 2 CMP ops, got {}", cmp_count);
}

#[test]
fn memcpy_preserves_pattern() {
    let memcpy = lib().get("memcpy").unwrap();
    // Should have both LOAD and STORE (memory copy pattern)
    assert!(memcpy.bytecodes.contains(&0x30), "memcpy missing LOAD");
    assert!(memcpy.bytecodes.contains(&0x31), "memcpy missing STORE");
}

#[test]
fn gcd_has_modulo() {
    let gcd = lib().get("gcd").unwrap();
    // Euclidean algorithm needs MOD (0x14)
    assert!(gcd.bytecodes.contains(&0x14), "gcd missing MOD opcode");
    // And a loop (JMP backwards or LOOP)
    assert!(gcd.bytecodes.contains(&0x50), "gcd missing JMP for loop");
}

#[test]
fn search_algorithms_have_compare() {
    let lib = lib();
    let ls = lib.get("linear_search").unwrap();
    let bs = lib.get("binary_search").unwrap();
    // Both should use CMP (0x20) and JEQ (0x55)
    assert!(ls.bytecodes.contains(&0x20) && ls.bytecodes.contains(&0x55));
    assert!(bs.bytecodes.contains(&0x20) && bs.bytecodes.contains(&0x55));
}

#[test]
fn sort_algorithms_have_swap() {
    let lib = lib();
    let bubble = lib.get("bubble_sort").unwrap();
    let selection = lib.get("selection_sort").unwrap();
    // Both should use STORE for swap operations
    assert!(bubble.bytecodes.contains(&0x31), "bubble_sort missing STORE");
    assert!(selection.bytecodes.contains(&0x31), "selection_sort missing STORE");
}

#[test]
fn agent_functions_use_load() {
    let lib = lib();
    assert!(lib.get("trust_check").unwrap().bytecodes.contains(&0x30));
    assert!(lib.get("energy_check").unwrap().bytecodes.contains(&0x30));
}

#[test]
fn all_function_names_unique() {
    let lib = lib();
    let names: Vec<_> = lib.all().map(|f| f.name.as_str()).collect();
    let mut sorted = names.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(names.len(), sorted.len(), "duplicate function names found");
}

fn lib() -> Stdlib {
    Stdlib::new()
}
