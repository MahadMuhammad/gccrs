//@ run-pass

#![warn(redundant_lifetimes)]

fn invariant_id<'a,'b>(t: &'b mut &'static ()) -> &'b mut &'a ()
// { dg-warning "" "" { target *-*-* } .-1 }
    where 'a: 'static { t }

fn static_id<'a>(t: &'a ()) -> &'static ()
// { dg-warning "" "" { target *-*-* } .-1 }
    where 'a: 'static { t }

fn static_id_indirect<'a,'b>(t: &'a ()) -> &'static ()
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    where 'a: 'b, 'b: 'static { t }

fn ref_id<'a>(t: &'a ()) -> &'a () where 'static: 'a { t }

static UNIT: () = ();

fn main()
{
    let mut val : &'static () = &UNIT;
    invariant_id(&mut val);
    static_id(val);
    static_id_indirect(val);
    ref_id(val);
}

