//@ check-pass

#![warn(non_local_definitions)]

trait Trait<T> {}

fn main() {
    mod below {
        pub struct Type<T>(T);
    }
    struct InsideMain;
    trait HasFoo {}

    impl<T> Trait<InsideMain> for &Vec<below::Type<(InsideMain, T)>>
// { dg-warning "" "" { target *-*-* } .-1 }
    where
        T: HasFoo
    {}
}

