struct Foo<'a>(&'a ());

fn test(_: Foo) {}

#[deny(elided_lifetimes_in_paths)]
mod w {
    fn test2(_: super::Foo) {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

