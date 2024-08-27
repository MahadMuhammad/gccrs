macro_rules! num { () => { 1 } }

fn main() {
    let x = 1i32;
    x.e10; // { dg-error ".E0610." "" { target *-*-* } }

    let y = 1;
    y.e10; // { dg-error ".E0610." "" { target *-*-* } }

    2u32.e10; // { dg-error ".E0610." "" { target *-*-* } }

    num!().e10; // { dg-error ".E0610." "" { target *-*-* } }

    2.e10foo; // { dg-error ".E0610." "" { target *-*-* } }

    42._;
// { dg-error ".E0610." "" { target *-*-* } .-1 }
// { dg-error ".E0610." "" { target *-*-* } .-2 }

    42.a; // { dg-error ".E0610." "" { target *-*-* } }
}

