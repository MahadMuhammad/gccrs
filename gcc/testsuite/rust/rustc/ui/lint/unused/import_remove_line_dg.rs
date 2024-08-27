//@ run-rustfix
//@ check-pass

#![crate_type = "lib"]
#![warn(unused_imports)]

use std::time::{Duration, Instant};
// { dg-warning "" "" { target *-*-* } .-1 }
use std::time::SystemTime;
// { dg-warning "" "" { target *-*-* } .-1 }
use std::time::SystemTimeError;use std::time::TryFromFloatSecsError;
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

