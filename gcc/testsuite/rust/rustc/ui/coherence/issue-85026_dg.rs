#![feature(auto_traits)]
auto trait AutoTrait {}

// You cannot impl your own `dyn AutoTrait`.
impl dyn AutoTrait {} // { dg-error ".E0785." "" { target *-*-* } }

// You cannot impl someone else's `dyn AutoTrait`
impl dyn Unpin {} // { dg-error ".E0785." "" { target *-*-* } }

fn main() {}

