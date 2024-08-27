struct Foo<'a, 'a> {
// { dg-error ".E0403." "" { target *-*-* } .-1 }
    x: &'a isize,
}

fn main() {}

