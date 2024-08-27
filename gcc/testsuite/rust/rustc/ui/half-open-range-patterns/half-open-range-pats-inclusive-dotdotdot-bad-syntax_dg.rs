// Test that `...X` range-to patterns are syntactically invalid.
//
// See https://github.com/rust-lang/rust/pull/67258#issuecomment-565656155
// for the reason why. To summarize, we might want to introduce `...expr` as
// an expression form for splatting (or "untupling") in an expression context.
// While there is no syntactic ambiguity with `...X` in a pattern context,
// there's a potential confusion factor here, and we would prefer to keep patterns
// and expressions in-sync. As such, we do not allow `...X` in patterns either.

fn main() {}

#[cfg(FALSE)]
fn syntax() {
    match scrutinee {
        ...X => {} // { dg-error "" "" { target *-*-* } }
        ...0 => {} // { dg-error "" "" { target *-*-* } }
        ...'a' => {} // { dg-error "" "" { target *-*-* } }
        ...0.0f32 => {} // { dg-error "" "" { target *-*-* } }
    }
}

fn syntax2() {
    macro_rules! mac {
        ($e:expr) => {
            let ...$e; // { dg-error ".E0005." "" { target *-*-* } }
// { dg-error ".E0005." "" { target *-*-* } .-1 }
        }
    }

    mac!(0);
}

