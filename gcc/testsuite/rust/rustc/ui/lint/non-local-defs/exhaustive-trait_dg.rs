//@ check-pass
// { dg-additional-options "-frust-edition=2021" }

#![warn(non_local_definitions)]

struct Dog;

fn main() {
    impl PartialEq<()> for Dog {
// { dg-warning "" "" { target *-*-* } .-1 }
        fn eq(&self, _: &()) -> bool {
            todo!()
        }
    }

    impl PartialEq<()> for &Dog {
// { dg-warning "" "" { target *-*-* } .-1 }
        fn eq(&self, _: &()) -> bool {
            todo!()
        }
    }

    impl PartialEq<Dog> for () {
// { dg-warning "" "" { target *-*-* } .-1 }
        fn eq(&self, _: &Dog) -> bool {
            todo!()
        }
    }

    impl PartialEq<&Dog> for () {
// { dg-warning "" "" { target *-*-* } .-1 }
        fn eq(&self, _: &&Dog) -> bool {
            todo!()
        }
    }

    impl PartialEq<Dog> for &Dog {
// { dg-warning "" "" { target *-*-* } .-1 }
        fn eq(&self, _: &Dog) -> bool {
            todo!()
        }
    }

    impl PartialEq<&Dog> for &Dog {
// { dg-warning "" "" { target *-*-* } .-1 }
        fn eq(&self, _: &&Dog) -> bool {
            todo!()
        }
    }
}

