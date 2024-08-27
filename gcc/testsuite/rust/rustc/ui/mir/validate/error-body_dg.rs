//@ compile-flags: -Zvalidate-mir

fn _test() {
    let x = || 45;
    missing();
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}

fn main() {}

