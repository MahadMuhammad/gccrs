// issue: rust-lang/rust#124022

struct Type<T>;
// { dg-error ".E0392." "" { target *-*-* } .-1 }

fn main() {
    {
        impl<T> Type<T> {
            fn new() -> Type<T> {
                Type
// { dg-error ".E0282." "" { target *-*-* } .-1 }
            }
        }
    };
}

