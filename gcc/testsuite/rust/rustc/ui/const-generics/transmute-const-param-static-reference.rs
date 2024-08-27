//@ revisions: full adt_const_params min
//@[full] check-pass

#![cfg_attr(full, feature(adt_const_params, unsized_const_params))]
#![cfg_attr(full, allow(incomplete_features))]
#![cfg_attr(adt_const_params, feature(adt_const_params))]
#![cfg_attr(adt_const_params, allow(incomplete_features))]

struct Const<const P: &'static ()>;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

fn main() {
    const A: &'static () = unsafe { std::mem::transmute(10 as *const ()) };

    let _ = Const::<{ A }>;
}

