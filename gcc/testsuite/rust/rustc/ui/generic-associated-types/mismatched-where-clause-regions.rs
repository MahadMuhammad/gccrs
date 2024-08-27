trait Foo {
    type T<'a1, 'b1>
    where
        'a1: 'b1;
}

impl Foo for () {
    type T<'a2, 'b2> = () where 'b2: 'a2;
// { dg-error ".E0276." "" { target *-*-* } .-1 }
}

fn main() {}

