#![feature(postfix_match)]

fn main() {
    Some(1).match { // { dg-error ".E0004." "" { target *-*-* } }
        None => {},
    }
}

