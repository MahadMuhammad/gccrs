trait Foo {
    fn test() -> impl Sized;
}

impl<'a, T> Foo for T {
// { dg-error ".E0207." "" { target *-*-* } .-1 }

    fn test() -> &'a () {
        &()
    }
}

fn main() {}

