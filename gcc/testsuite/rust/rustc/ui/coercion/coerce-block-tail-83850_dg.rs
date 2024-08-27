//@ check-fail
fn f(_: &[i32]) {}

fn main() {
    f(&Box::new([1, 2]));
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

