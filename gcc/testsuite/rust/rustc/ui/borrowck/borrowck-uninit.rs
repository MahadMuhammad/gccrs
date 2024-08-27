fn foo(x: isize) { println!("{}", x); }

fn main() {
    let x: isize;
    foo(x); // { dg-error ".E0381." "" { target *-*-* } }

    // test for #120634
    struct A(u8);
    struct B { d: u8 }
    let (a, );
    let [b, ];
    let A(c);
    let B { d };
    let _: (u8, u8, u8, u8) = (a, b, c, d);
// { dg-error ".E0381." "" { target *-*-* } .-1 }
// { dg-error ".E0381." "" { target *-*-* } .-2 }
// { dg-error ".E0381." "" { target *-*-* } .-3 }
// { dg-error ".E0381." "" { target *-*-* } .-4 }
}

