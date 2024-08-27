#![allow(dead_code)]
struct U <T> {
    wtf: Option<Box<U<T>>>,
    x: T,
}
fn main() {
    U {
        wtf: Some(Box(U { // { dg-error ".E0423." "" { target *-*-* } }
            wtf: None,
            x: (),
        })),
        x: ()
    };
    let _ = std::collections::HashMap();
// { dg-error ".E0423." "" { target *-*-* } .-1 }
    let _ = std::collections::HashMap {};
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = Box {}; // { dg-error "" "" { target *-*-* } }
}

