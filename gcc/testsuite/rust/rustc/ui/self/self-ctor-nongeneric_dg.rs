// `Self` as a constructor is currently allowed when the outer item is not generic.
//@ check-pass

struct S0(usize);

impl S0 {
    fn foo() {
        const C: S0 = Self(0);
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
        fn bar() -> S0 {
            Self(0)
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
        }
    }
}

fn main() {}

