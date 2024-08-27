// The trait must not be available if its feature flag is absent.

#![crate_type = "lib"]

use std::mem::TransmuteFrom;
// { dg-error ".E0658." "" { target *-*-* } .-1 }

use std::mem::Assume;
// { dg-error ".E0658." "" { target *-*-* } .-1 }

