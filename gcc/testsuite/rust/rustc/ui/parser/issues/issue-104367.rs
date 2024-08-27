#[derive(A)]
struct S {
    d: [u32; {
        #![cfg] {
            #![w,) // { dg-error "" "" { target *-*-* } }
                   // { dg-error "" "" { target *-*-* } }

