//@ check-fail
//@ run-rustfix

#![deny(forgetting_references)]

use std::mem::forget;

struct SomeStruct;

fn main() {
    forget(&SomeStruct); // { dg-error "" "" { target *-*-* } }

    let mut owned = SomeStruct;
    forget(&owned); // { dg-error "" "" { target *-*-* } }
    forget(&&owned); // { dg-error "" "" { target *-*-* } }
    forget(&mut owned); // { dg-error "" "" { target *-*-* } }
    forget(owned);

    let reference1 = &SomeStruct;
    forget(&*reference1); // { dg-error "" "" { target *-*-* } }

    let reference2 = &mut SomeStruct;
    forget(reference2); // { dg-error "" "" { target *-*-* } }

    let ref reference3 = SomeStruct;
    forget(reference3); // { dg-error "" "" { target *-*-* } }
}

#[allow(dead_code)]
fn test_generic_fn_forget<T>(val: T) {
    forget(&val); // { dg-error "" "" { target *-*-* } }
    forget(val);
}

#[allow(dead_code)]
fn test_similarly_named_function() {
    fn forget<T>(_val: T) {}
    forget(&SomeStruct); //OK; call to unrelated function which happens to have the same name
    std::mem::forget(&SomeStruct); // { dg-error "" "" { target *-*-* } }
}

