trait Foo {
    type Bar<,>;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

