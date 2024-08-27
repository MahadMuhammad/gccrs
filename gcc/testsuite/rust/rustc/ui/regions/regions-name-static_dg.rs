struct Foo<'static> {
// { dg-error ".E0262." "" { target *-*-* } .-1 }
    x: &'static isize,
}

fn main() {}

