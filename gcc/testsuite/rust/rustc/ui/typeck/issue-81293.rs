fn main() {
    let a: u16;
    let b: u16 = 42;
    let c: usize = 5;

    a = c + b * 5; // { dg-error ".E0277." "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
}

