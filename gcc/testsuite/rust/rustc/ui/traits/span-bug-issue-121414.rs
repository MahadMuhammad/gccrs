trait Bar {
    type Type;
}
struct Foo<'a>(&'a ());
impl<'a> Bar for Foo<'f> { // { dg-error ".E0261." "" { target *-*-* } }
    type Type = u32;
}

fn test() // { dg-error ".E0277." "" { target *-*-* } }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
where
    for<'a> <Foo<'a> as Bar>::Type: Sized,
{
}

fn main() {}

