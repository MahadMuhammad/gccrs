// https://github.com/rust-lang/rust/issues/123844
// An uninitialized refutable let should not suggest `let else`, as it can't be used with deferred
// initialization.

fn main() {
    let Some(x); // { dg-error ".E0005." "" { target *-*-* } }
    x = 1;
}

