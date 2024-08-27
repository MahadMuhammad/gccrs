//@check-fail

#![crate_type="lib"]

fn r({) { // { dg-error "" "" { target *-*-* } }
    Ok {
        d..||_=m
    }
}

