#![feature(postfix_match)]

fn main() {
    Some(10).match {
// { dg-note ".E0308." "" { target *-*-* } .-1 }
        Some(5) => false,
// { dg-note "" "" { target *-*-* } .-1 }
        Some(2) => true,
// { dg-note "" "" { target *-*-* } .-1 }
        None    => (),
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
        _       => true
    }
}

