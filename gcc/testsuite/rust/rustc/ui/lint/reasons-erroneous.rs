//@ compile-flags: -Zdeduplicate-diagnostics=yes

#![warn(absolute_paths_not_starting_with_crate, reason = 0)]
// { dg-error ".E0452." "" { target *-*-* } .-1 }
// { dg-note ".E0452." "" { target *-*-* } .-2 }
#![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
// { dg-error ".E0452." "" { target *-*-* } .-1 }
// { dg-note ".E0452." "" { target *-*-* } .-2 }
#![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
// { dg-error ".E0452." "" { target *-*-* } .-1 }
// { dg-note ".E0452." "" { target *-*-* } .-2 }
#![warn(unsafe_code, blerp = "or in league with robbers have reversed the signposts")]
// { dg-error ".E0452." "" { target *-*-* } .-1 }
// { dg-note ".E0452." "" { target *-*-* } .-2 }
#![warn(elided_lifetimes_in_paths, reason("disrespectful to ancestors", "irresponsible to heirs"))]
// { dg-error ".E0452." "" { target *-*-* } .-1 }
// { dg-note ".E0452." "" { target *-*-* } .-2 }
#![warn(ellipsis_inclusive_range_patterns, reason = "born barren", reason = "a freak growth")]
// { dg-error ".E0452." "" { target *-*-* } .-1 }
// { dg-note ".E0452." "" { target *-*-* } .-2 }
#![warn(keyword_idents, reason = "root in rubble", macro_use_extern_crate)]
// { dg-error ".E0452." "" { target *-*-* } .-1 }
// { dg-note ".E0452." "" { target *-*-* } .-2 }
#![warn(missing_copy_implementations, reason)]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

fn main() {}

