fn main() {
    let box = 0;
// { dg-error "" "" { target *-*-* } .-1 }
    let box: bool;
// { dg-error "" "" { target *-*-* } .-1 }
    let mut box = 0;
// { dg-error "" "" { target *-*-* } .-1 }
    let (box,) = (0,);
// { dg-error "" "" { target *-*-* } .-1 }
}

