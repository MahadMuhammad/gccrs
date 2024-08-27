fn main() {
    let x = "x";
    let y = "y";

    println!("{x}", x, x = y);
// { dg-error "" "" { target *-*-* } .-1 }

    println!("{x}", x = y, x = y);
// { dg-error "" "" { target *-*-* } .-1 }
}

