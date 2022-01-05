//! Pass timing.
//!
//! This modules provides facilities for timing the execution of individual compilation passes.

use core::fmt;

pub use self::details::{add_to_current, take_current, PassTimes, TimingToken};

// Each pass that can be timed is predefined with the `define_passes!` macro. Each pass has a
// snake_case name and a plain text description used when printing out the timing report.
//
// This macro defines:
//
// - A C-style enum containing all the pass names and a `None` variant.
// - A usize constant with the number of defined passes.
// - A const array of pass descriptions.
// - A public function per pass used to start the timing of that pass.
macro_rules! define_passes {
    { $enum:ident, $num_passes:ident, $descriptions:ident;
      $($pass:ident: $desc:expr,)+
    } => {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        enum $enum { $($pass,)+ None}

        const $num_passes: usize = $enum::None as usize;

        const $descriptions: [&str; $num_passes] = [ $($desc),+ ];

        $(
            #[doc=$desc]
            pub fn $pass() -> TimingToken {
                details::start_pass($enum::$pass)
            }
        )+
    }
}

// Pass definitions.
define_passes! {
    Pass, NUM_PASSES, DESCRIPTIONS;

    process_file: "Processing test file",
    parse_text: "Parsing textual Cranelift IR",
    wasm_translate_module: "Translate WASM module",
    wasm_translate_function: "Translate WASM function",

    verifier: "Verify Cranelift IR",
    verify_cssa: "Verify CSSA",
    verify_liveness: "Verify live ranges",
    verify_locations: "Verify value locations",
    verify_flags: "Verify CPU flags",

    compile: "Compilation passes",
    flowgraph: "Control flow graph",
    domtree: "Dominator tree",
    loop_analysis: "Loop analysis",
    postopt: "Post-legalization rewriting",
    preopt: "Pre-legalization rewriting",
    dce: "Dead code elimination",
    legalize: "Legalization",
    gvn: "Global value numbering",
    licm: "Loop invariant code motion",
    unreachable_code: "Remove unreachable blocks",
    remove_constant_phis: "Remove constant phi-nodes",

    vcode_lower: "VCode lowering",
    vcode_post_ra: "VCode post-register allocation finalization",
    vcode_emit: "VCode emission",
    vcode_emit_finish: "VCode emission finalization",

    regalloc: "Register allocation",
    ra_liveness: "RA liveness analysis",
    ra_cssa: "RA coalescing CSSA",
    ra_spilling: "RA spilling",
    ra_reload: "RA reloading",
    ra_coloring: "RA coloring",

    prologue_epilogue: "Prologue/epilogue insertion",
    shrink_instructions: "Instruction encoding shrinking",
    relax_branches: "Branch relaxation",
    binemit: "Binary machine code emission",
    layout_renumber: "Layout full renumbering",

    canonicalize_nans: "Canonicalization of NaNs",
}

impl Pass {
    pub fn idx(self) -> usize {
        self as usize
    }
}

impl fmt::Display for Pass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match DESCRIPTIONS.get(self.idx()) {
            Some(s) => f.write_str(s),
            None => f.write_str("<no pass>"),
        }
    }
}

/// Dummy `debug` implementation
mod details {
    use super::Pass;
    /// Dummy `TimingToken`
    pub struct TimingToken;
    /// Dummy `PassTimes`
    pub struct PassTimes;
    /// Returns dummy `PassTimes`
    pub fn take_current() -> PassTimes {
        PassTimes
    }
    /// does nothing
    pub fn add_to_current(_times: PassTimes) {}

    /// does nothing
    pub(super) fn start_pass(_pass: Pass) -> TimingToken {
        TimingToken
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn display() {
        assert_eq!(Pass::None.to_string(), "<no pass>");
        assert_eq!(Pass::regalloc.to_string(), "Register allocation");
    }
}
