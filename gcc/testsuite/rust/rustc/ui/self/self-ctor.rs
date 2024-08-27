struct S0<T>(T);

impl<T> S0<T> {
    fn foo() {
        const C: S0<i32> = Self(0);
// { dg-error ".E0401." "" { target *-*-* } .-1 }
        fn bar() -> S0<i32> {
            Self(0)
// { dg-error ".E0401." "" { target *-*-* } .-1 }
        }
    }
}

fn main() {}

