macro_rules! foo {
    ($ty:ty) => {
        fn foo(_: $ty, _: $ty) {}
    }
}

foo!(_);
// { dg-error ".E0121." "" { target *-*-* } .-1 }

fn main() {}

