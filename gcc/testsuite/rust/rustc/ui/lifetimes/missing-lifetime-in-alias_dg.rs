trait Trait<'a> {
    type Foo;

    type Bar<'b>
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    where
        Self: 'b;
}

struct Impl<'a>(&'a ());

impl<'a> Trait<'a> for Impl<'a> {
    type Foo = &'a ();
    type Bar<'b> = &'b ();
// { dg-error ".E0477." "" { target *-*-* } .-1 }
// { dg-note ".E0477." "" { target *-*-* } .-2 }
}

type A<'a> = Impl<'a>;

type B<'a> = <A<'a> as Trait>::Foo;
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { dg-note ".E0106." "" { target *-*-* } .-2 }

type C<'a, 'b> = <A<'a> as Trait>::Bar;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
// { dg-note ".E0107." "" { target *-*-* } .-3 }
// { dg-note ".E0107." "" { target *-*-* } .-4 }
// { dg-note ".E0107." "" { target *-*-* } .-5 }

fn main() {}

