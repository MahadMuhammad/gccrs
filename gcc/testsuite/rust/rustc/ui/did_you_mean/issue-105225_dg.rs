//@ run-rustfix

fn main() {
    let x = "x";
    let y = "y";

    println!("{x}", x);
// { dg-error "" "" { target *-*-* } .-1 }

    println!("{x} {}", x, x);
// { dg-error "" "" { target *-*-* } .-1 }

    println!("{} {x}", x, x);
// { dg-error "" "" { target *-*-* } .-1 }

    println!("{x} {y}", x, y);
// { dg-error "" "" { target *-*-* } .-1 }

    println!("{} {} {x} {y} {}", x, x, x, y, y);
// { dg-error "" "" { target *-*-* } .-1 }
}

