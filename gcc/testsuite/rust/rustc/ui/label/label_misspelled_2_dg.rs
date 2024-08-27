#![warn(unused_labels)]

fn main() {
    'a: for _ in 0..1 {
        break 'a;
    }
    'b: for _ in 0..1 {
        break b; // { dg-error ".E0425." "" { target *-*-* } }
    }
    c: for _ in 0..1 { // { dg-error "" "" { target *-*-* } }
        break 'c;
    }
    d: for _ in 0..1 { // { dg-error "" "" { target *-*-* } }
        break d; // { dg-error ".E0425." "" { target *-*-* } }
    }
}

