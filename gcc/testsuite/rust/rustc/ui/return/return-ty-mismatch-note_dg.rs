// Checks existence or absence of a note for "a caller chooses ty for ty param" upon return ty
// mismatch.

fn f<T>() -> (T,) {
    (0,) // { dg-error ".E0308." "" { target *-*-* } }
}

fn g<U, V>() -> (U, V) {
    (0, "foo")
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
}

fn h() -> u8 {
    0u8
}

// This case was reported in <https://github.com/rust-lang/rust/issues/126547> where it doesn't
// make sense to make the "note caller chooses ty for ty param" note if the found type contains
// the ty param...
fn k<T>(_t: &T) -> T {
    _t
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {
    f::<()>();
    g::<(), ()>;
    let _ = h();
}

