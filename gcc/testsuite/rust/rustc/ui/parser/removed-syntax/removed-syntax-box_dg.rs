//@ run-rustfix

fn main() {
    #[allow(dead_code)]
    struct T {
        a: u8,
        b: u8,
    }
    let _ = box (); // { dg-error "" "" { target *-*-* } }
    let _ = box 1; // { dg-error "" "" { target *-*-* } }
    let _ = box T { a: 12, b: 18 }; // { dg-error "" "" { target *-*-* } }
    let _ = box [5; 30]; // { dg-error "" "" { target *-*-* } }
    let _: Box<()> = box (); // { dg-error "" "" { target *-*-* } }
}

