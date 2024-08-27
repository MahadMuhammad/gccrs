//@ check-pass
// { dg-additional-options "-frust-edition=2021" }

#![warn(non_local_definitions)]

use std::fmt::Display;

trait Trait {}
struct Test;

fn main() {
    impl Test {
// { dg-warning "" "" { target *-*-* } .-1 }
        fn foo() {}
    }

    impl Display for Test {
// { dg-warning "" "" { target *-*-* } .-1 }
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl dyn Trait {}
// { dg-warning "" "" { target *-*-* } .-1 }

    impl<T: Trait> Trait for Vec<T> { }
// { dg-warning "" "" { target *-*-* } .-1 }

    impl Trait for &dyn Trait {}
// { dg-warning "" "" { target *-*-* } .-1 }

    impl Trait for *mut Test {}
// { dg-warning "" "" { target *-*-* } .-1 }

    impl Trait for *mut [Test] {}
// { dg-warning "" "" { target *-*-* } .-1 }

    impl Trait for [Test; 8] {}
// { dg-warning "" "" { target *-*-* } .-1 }

    impl Trait for (Test,) {}
// { dg-warning "" "" { target *-*-* } .-1 }

    impl Trait for fn(Test) -> () {}
// { dg-warning "" "" { target *-*-* } .-1 }

    impl Trait for fn() -> Test {}
// { dg-warning "" "" { target *-*-* } .-1 }

    let _a = || {
        impl Trait for Test {}
// { dg-warning "" "" { target *-*-* } .-1 }

        1
    };

    struct InsideMain;

    impl Trait for *mut InsideMain {}
// { dg-warning "" "" { target *-*-* } .-1 }
    impl Trait for *mut [InsideMain] {}
// { dg-warning "" "" { target *-*-* } .-1 }
    impl Trait for [InsideMain; 8] {}
// { dg-warning "" "" { target *-*-* } .-1 }
    impl Trait for (InsideMain,) {}
// { dg-warning "" "" { target *-*-* } .-1 }
    impl Trait for fn(InsideMain) -> () {}
// { dg-warning "" "" { target *-*-* } .-1 }
    impl Trait for fn() -> InsideMain {}
// { dg-warning "" "" { target *-*-* } .-1 }

    fn inside_inside() {
        impl Display for InsideMain {
// { dg-warning "" "" { target *-*-* } .-1 }
            fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                todo!()
            }
        }

        impl InsideMain {
// { dg-warning "" "" { target *-*-* } .-1 }
            fn bar() {}
        }
    }
}

