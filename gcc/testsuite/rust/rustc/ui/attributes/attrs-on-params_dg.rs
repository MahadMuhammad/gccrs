// This checks that incorrect params on function parameters are caught

fn function(#[inline] param: u32) {
// { dg-error ".E0518." "" { target *-*-* } .-1 }
// { dg-error ".E0518." "" { target *-*-* } .-2 }
}

fn main() {}

