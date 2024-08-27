#![feature(if_let_guard)]

#![deny(irrefutable_let_patterns)]

fn main() {
    if let _ = 5 {} // { dg-error "" "" { target *-*-* } }

    while let _ = 5 { // { dg-error "" "" { target *-*-* } }
        break;
    }

    match 5 {
        _ if let _ = 2 => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
}

