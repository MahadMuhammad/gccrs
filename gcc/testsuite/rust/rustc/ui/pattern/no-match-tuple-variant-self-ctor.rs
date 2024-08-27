//@ revisions: tuple unit struct_
//@[unit] check-pass

#[cfg(unit)]
mod unit {
    struct S;
    impl S {
        fn foo() {
            let Self = S;
        }
    }
}

#[cfg(tuple)]
mod tuple {
    struct S(());
    impl S {
        fn foo() {
            let Self = S;
// { dg-error "" "" { target *-*-* } .-1 }
        }
    }
}

#[cfg(struct_)]
mod struct_ {
    struct S {}
    impl S {
        fn foo() {
            let Self = S;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        }
    }
}

fn main() {}

