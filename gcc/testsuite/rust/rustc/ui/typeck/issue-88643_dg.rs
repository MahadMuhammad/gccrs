// Regression test for the ICE described in #88643. Specifically:
// https://github.com/rust-lang/rust/issues/88643#issuecomment-913128893
// and https://github.com/rust-lang/rust/issues/88643#issuecomment-913171935
// and https://github.com/rust-lang/rust/issues/88643#issuecomment-913765984

use std::collections::HashMap;

pub trait T {}

static CALLBACKS: HashMap<*const dyn T, dyn FnMut(&mut _) + 'static> = HashMap::new();
// { dg-error ".E0121." "" { target *-*-* } .-1 }

static CALLBACKS2: Vec<dyn Fn(& _)> = Vec::new();
// { dg-error ".E0121." "" { target *-*-* } .-1 }

static CALLBACKS3: Option<dyn Fn(& _)> = None;
// { dg-error ".E0121." "" { target *-*-* } .-1 }

fn main() {}

