//@ run-rustfix
fn main() {
    let _ = vec![true, false].map(|v| !v).collect::<Vec<_>>();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

