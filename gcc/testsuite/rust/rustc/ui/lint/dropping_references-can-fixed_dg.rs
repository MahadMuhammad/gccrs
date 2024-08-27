//@ check-fail
//@ run-rustfix

#![deny(dropping_references)]

struct SomeStruct;

fn main() {
    drop(&SomeStruct); // { dg-error "" "" { target *-*-* } }

    let mut owned1 = SomeStruct;
    drop(&owned1); // { dg-error "" "" { target *-*-* } }
    drop(&&owned1); // { dg-error "" "" { target *-*-* } }
    drop(&mut owned1); // { dg-error "" "" { target *-*-* } }
    drop(owned1);

    let reference1 = &SomeStruct;
    drop(reference1); // { dg-error "" "" { target *-*-* } }

    let reference2 = &mut SomeStruct;
    drop(reference2); // { dg-error "" "" { target *-*-* } }

    let ref reference3 = SomeStruct;
    drop(reference3); // { dg-error "" "" { target *-*-* } }
}

#[allow(dead_code)]
fn test_generic_fn_drop<T>(val: T) {
    drop(&val); // { dg-error "" "" { target *-*-* } }
    drop(val);
}

