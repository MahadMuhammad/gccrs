//@ check-pass
// { dg-additional-options "-frust-edition=2021" }
//@ rustc-env:CARGO_CRATE_NAME=non_local_def

#![warn(non_local_definitions)]

struct Test;

trait Uto {}
const Z: () = {
    trait Uto1 {}

    impl Uto1 for Test {} // the trait is local, don't lint

    impl Uto for &Test {}
// { dg-warning "" "" { target *-*-* } .-1 }
};

trait Ano {}
const _: () = {
    impl Ano for &Test {} // ignored since the parent is an anon-const
};

trait Uto2 {}
static A: u32 = {
    impl Uto2 for Test {}
// { dg-warning "" "" { target *-*-* } .-1 }

    1
};

trait Uto3 {}
const B: u32 = {
    impl Uto3 for Test {}
// { dg-warning "" "" { target *-*-* } .-1 }

    trait Uto4 {}
    impl Uto4 for Test {}

    1
};

trait Uto5 {}
fn main() {
    impl Test {
// { dg-warning "" "" { target *-*-* } .-1 }
        fn foo() {}
    }


    const {
        impl Test {
// { dg-warning "" "" { target *-*-* } .-1 }
            fn hoo() {}
        }

        1
    };

    const _: u32 = {
        impl Test {
// { dg-warning "" "" { target *-*-* } .-1 }
            fn foo2() {}
        }

        1
    };
}

trait Uto9 {}
trait Uto10 {}
const _: u32 = {
    let _a = || {
        impl Uto9 for Test {}
// { dg-warning "" "" { target *-*-* } .-1 }

        1
    };

    type A = [u32; {
        impl Uto10 for Test {}
// { dg-warning "" "" { target *-*-* } .-1 }

        1
    }];

    1
};

