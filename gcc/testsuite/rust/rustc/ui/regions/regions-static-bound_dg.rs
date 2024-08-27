#![warn(unused_lifetimes, redundant_lifetimes)]

fn static_id<'a,'b>(t: &'a ()) -> &'static () where 'a: 'static { t }
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

fn static_id_indirect<'a,'b>(t: &'a ()) -> &'static ()
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    where 'a: 'b, 'b: 'static { t }

fn static_id_wrong_way<'a>(t: &'a ()) -> &'static () where 'static: 'a {
    t
// { dg-error "" "" { target *-*-* } .-1 }
}

fn error(u: &(), v: &()) {
    static_id(&u);
// { dg-error ".E0521." "" { target *-*-* } .-1 }
    static_id_indirect(&v);
// { dg-error ".E0521." "" { target *-*-* } .-1 }
}

fn main() {}

