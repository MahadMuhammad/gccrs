// Regression test for https://github.com/rust-lang/rust/issues/129205
fn x<T: Copy>() {
    T::try_from(); // { dg-error ".E0599." "" { target *-*-* } }
}

fn main() {}

