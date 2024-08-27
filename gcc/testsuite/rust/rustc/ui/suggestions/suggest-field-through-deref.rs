//@ run-rustfix
#![allow(dead_code)]
use std::sync::Arc;
struct S {
    long_name: (),
    foo: (),
}
fn main() {
    let x = Arc::new(S { long_name: (), foo: () });
    let _ = x.longname; // { dg-error ".E0609." "" { target *-*-* } }
    let y = S { long_name: (), foo: () };
    let _ = y.longname; // { dg-error ".E0609." "" { target *-*-* } }
    let a = Some(Arc::new(S { long_name: (), foo: () }));
    let _ = a.longname; // { dg-error ".E0609." "" { target *-*-* } }
    let b = Some(S { long_name: (), foo: () });
    let _ = b.long_name; // { dg-error ".E0609." "" { target *-*-* } }
    let c = Ok::<_, ()>(Arc::new(S { long_name: (), foo: () }));
    let _ = c.longname; // { dg-error ".E0609." "" { target *-*-* } }
    let d = Ok::<_, ()>(S { long_name: (), foo: () });
    let _ = d.long_name; // { dg-error ".E0609." "" { target *-*-* } }
}

