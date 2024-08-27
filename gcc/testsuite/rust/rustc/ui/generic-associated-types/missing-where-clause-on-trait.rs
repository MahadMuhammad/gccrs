//@ check-fail

trait Foo {
    type Assoc<'a, 'b>;
}
impl Foo for () {
    type Assoc<'a, 'b> = () where 'a: 'b;
// { dg-error ".E0276." "" { target *-*-* } .-1 }
}

fn main() {}

