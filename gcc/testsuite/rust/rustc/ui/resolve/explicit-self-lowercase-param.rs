struct Foo;

impl Foo {
    fn do_nothing(self: Box<self>) {} // { dg-error "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
}

fn main() {}

