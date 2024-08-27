struct Foo<T>(T);

impl<'s> Foo<&'s u8> {
    fn bar<'s>(&self, x: &'s u8) {} // { dg-error ".E0496." "" { target *-*-* } }
    fn baz(x: for<'s> fn(&'s u32)) {} // { dg-error ".E0496." "" { target *-*-* } }
}

fn main() {}

