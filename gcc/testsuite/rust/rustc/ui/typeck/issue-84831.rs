fn f() {
    std::<0>; // { dg-error ".E0423." "" { target *-*-* } }
}
fn j() {
    std::<_ as _>; // { dg-error ".E0423." "" { target *-*-* } }
// { dg-error ".E0423." "" { target *-*-* } .-1 }
}

fn main () {}

