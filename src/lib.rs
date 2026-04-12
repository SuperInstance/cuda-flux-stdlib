//! # cuda-flux-stdlib
//!
//! Standard library for FLUX VM programs — a collection of useful bytecode
//! subroutine patterns defined as Rust constants.
//!
//! "The Deeper Connection."

mod math;
mod memory;
mod search;
mod sorting;
mod agent;

/// A standard library function: a named bytecode subroutine with metadata.
#[derive(Debug, Clone)]
pub struct StdlibFunction {
    pub name: String,
    pub bytecodes: Vec<u8>,
    pub param_count: u8,
    pub return_count: u8,
    pub stack_usage: u8,
    pub description: String,
}

/// The standard library registry.
pub struct Stdlib {
    functions: Vec<StdlibFunction>,
}

impl Stdlib {
    pub fn new() -> Self {
        let mut s = Self { functions: Vec::new() };
        s.register(math::register());
        s.register(memory::register());
        s.register(search::register());
        s.register(sorting::register());
        s.register(agent::register());
        s
    }

    fn register(&mut self, fns: Vec<StdlibFunction>) {
        self.functions.extend(fns);
    }

    pub fn get(&self, name: &str) -> Option<&StdlibFunction> {
        self.functions.iter().find(|f| f.name == name)
    }

    pub fn all(&self) -> impl Iterator<Item = &StdlibFunction> {
        self.functions.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stdlib_contains_all_modules() {
        let lib = Stdlib::new();
        assert!(lib.get("abs").is_some());
        assert!(lib.get("max").is_some());
        assert!(lib.get("min").is_some());
        assert!(lib.get("clamp").is_some());
        assert!(lib.get("swap").is_some());
        assert!(lib.get("power").is_some());
        assert!(lib.get("gcd").is_some());
        assert!(lib.get("memset").is_some());
        assert!(lib.get("memcpy").is_some());
        assert!(lib.get("memcmp").is_some());
        assert!(lib.get("reverse").is_some());
        assert!(lib.get("linear_search").is_some());
        assert!(lib.get("binary_search").is_some());
        assert!(lib.get("bubble_sort").is_some());
        assert!(lib.get("selection_sort").is_some());
        assert!(lib.get("trust_check").is_some());
        assert!(lib.get("energy_check").is_some());
        assert!(lib.get("should_delegate").is_some());
        assert!(lib.get("report_status").is_some());
    }
}
