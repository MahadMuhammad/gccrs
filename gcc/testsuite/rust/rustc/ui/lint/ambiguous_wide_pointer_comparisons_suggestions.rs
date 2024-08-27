//@ run-rustfix
//@ rustfix-only-machine-applicable
//@ check-pass

// See <https://github.com/rust-lang/rust/issues/121330>.

fn cmp<T: ?Sized>(a: *mut T, b: *mut T) -> bool {
    let _ = a == b;
// { dg-warning "" "" { target *-*-* } .-1 }
    panic!();
}

fn main() {}

