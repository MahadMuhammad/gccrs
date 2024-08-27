//@ check-pass

#![warn(forgetting_references)]

use std::mem::forget;

struct SomeStruct;

fn main() {
    forget(&SomeStruct); // { dg-warning "" "" { target *-*-* } }

    let mut owned = SomeStruct;
    forget(&owned); // { dg-warning "" "" { target *-*-* } }
    forget(&&owned); // { dg-warning "" "" { target *-*-* } }
    forget(&mut owned); // { dg-warning "" "" { target *-*-* } }
    forget(owned);

    let reference1 = &SomeStruct;
    forget(&*reference1); // { dg-warning "" "" { target *-*-* } }

    let reference2 = &mut SomeStruct;
    forget(reference2); // { dg-warning "" "" { target *-*-* } }

    let ref reference3 = SomeStruct;
    forget(reference3); // { dg-warning "" "" { target *-*-* } }

    let ref reference4 = SomeStruct;

    let a = 1;
    match a {
        1 => forget(&*reference1), // { dg-warning "" "" { target *-*-* } }
        2 => forget(reference3), // { dg-warning "" "" { target *-*-* } }
        3 => forget(reference4), // { dg-warning "" "" { target *-*-* } }
        _ => {}
    }
}

#[allow(dead_code)]
fn test_generic_fn_forget<T>(val: T) {
    forget(&val); // { dg-warning "" "" { target *-*-* } }
    forget(val);
}

#[allow(dead_code)]
fn test_similarly_named_function() {
    fn forget<T>(_val: T) {}
    forget(&SomeStruct); //OK; call to unrelated function which happens to have the same name
    std::mem::forget(&SomeStruct); // { dg-warning "" "" { target *-*-* } }
}

