#[derive(Debug)]
struct Foo {
    #[cfg(all())]
    field: fn(($),), // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

