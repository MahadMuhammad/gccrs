trait Foo {
    type F<'a>;

    fn identity<'a>(t: &'a Self::F<'a>) -> &'a Self::F<'a> { t }
}

impl <T, T1> Foo for T {
// { dg-error ".E0207." "" { target *-*-* } .-1 }
    type F<T1> = &[u8];
// { dg-error ".E0637." "" { target *-*-* } .-1 }
// { dg-error ".E0637." "" { target *-*-* } .-2 }
}

fn main() {}

