fn main() {
    let x: fn~() = || (); // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }
}

