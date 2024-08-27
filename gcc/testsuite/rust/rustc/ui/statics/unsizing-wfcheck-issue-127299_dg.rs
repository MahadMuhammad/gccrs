// This ensures we don't ICE in situations like rust-lang/rust#127299.

trait Qux {
    fn bar() -> i32;
}

pub struct Lint {
    pub desc: &'static dyn Qux,
// { dg-error ".E0038." "" { target *-*-* } .-1 }
}

static FOO: &Lint = &Lint { desc: "desc" };
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }

fn main() {}

