#![feature(postfix_match)]

fn main() {
    1 as i32.match {};
// { dg-error ".E0004." "" { target *-*-* } .-1 }
// { dg-error ".E0004." "" { target *-*-* } .-2 }
}

