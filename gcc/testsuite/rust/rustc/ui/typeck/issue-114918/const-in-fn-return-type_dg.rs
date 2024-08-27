// Regression test for #114918
// Test that a const generic enclosed in a block within a return type
// produces a type mismatch error instead of triggering a const eval cycle

#[allow(unused_braces)]
fn func() -> [u8; { () } ] { // { dg-error ".E0308." "" { target *-*-* } }
    loop {}
}

fn main() {}

