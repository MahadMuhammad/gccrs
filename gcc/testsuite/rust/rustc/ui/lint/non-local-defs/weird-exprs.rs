//@ check-pass
// { dg-additional-options "-frust-edition=2021" }

#![warn(non_local_definitions)]

trait Uto {}
struct Test;

type A = [u32; {
    impl Uto for *mut Test {}
// { dg-warning "" "" { target *-*-* } .-1 }

    1
}];

enum Enum {
    Discr = {
        impl Uto for Test {}
// { dg-warning "" "" { target *-*-* } .-1 }

        1
    }
}

fn main() {
    let _array = [0i32; {
        impl Test {
// { dg-warning "" "" { target *-*-* } .-1 }
            fn bar() {}
        }

        1
    }];

    type A = [u32; {
        impl Uto for &Test {}
// { dg-warning "" "" { target *-*-* } .-1 }

        1
    }];

    fn a(_: [u32; {
        impl Uto for &(Test,) {}
// { dg-warning "" "" { target *-*-* } .-1 }

        1
    }]) {}

    fn b() -> [u32; {
        impl Uto for &(Test,Test) {}
// { dg-warning "" "" { target *-*-* } .-1 }

        1
    }] { todo!() }
}

