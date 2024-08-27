//! This test ensures we do not ICE for unimplemented
//! patterns unless the feature gate is enabled.

#![feature(core_pattern_type)]
#![feature(core_pattern_types)]

use std::pat::pattern_type;

type Always = pattern_type!(Option<u32> is Some(_));
// { dg-error ".E0658." "" { target *-*-* } .-1 }

type Binding = pattern_type!(Option<u32> is x);
// { dg-error ".E0658." "" { target *-*-* } .-1 }

fn main() {}

