//! Ensure we don't allow accessing const effect parameters from stable Rust.

fn main() {
    i8::checked_sub::<true>(42, 43);
// { dg-error ".E0107." "" { target *-*-* } .-1 }
    i8::checked_sub::<false>(42, 43);
// { dg-error ".E0107." "" { target *-*-* } .-1 }
}

const FOO: () = {
    i8::checked_sub::<false>(42, 43);
// { dg-error ".E0107." "" { target *-*-* } .-1 }
    i8::checked_sub::<true>(42, 43);
// { dg-error ".E0107." "" { target *-*-* } .-1 }
};

