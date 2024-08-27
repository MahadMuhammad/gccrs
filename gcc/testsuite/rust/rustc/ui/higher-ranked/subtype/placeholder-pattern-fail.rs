// Check that incorrect higher ranked subtyping
// causes an error.
struct Inv<'a>(fn(&'a ()) -> &'a ());
fn hr_subtype<'c>(f: for<'a, 'b> fn(Inv<'a>, Inv<'a>)) {
    // ok
    let _: for<'a> fn(Inv<'a>, Inv<'a>) = f;
    let sub: for<'a> fn(Inv<'a>, Inv<'a>) = f;
    // no
    let _: for<'a, 'b> fn(Inv<'a>, Inv<'b>) = sub;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn simple1<'c>(x: (&'c i32,)) {
    let _x: (&'static i32,) = x;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn simple2<'c>(x: (&'c i32,)) {
    let _: (&'static i32,) = x;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    hr_subtype(|_, _| {});
    simple1((&3,));
    simple2((&3,));
}

