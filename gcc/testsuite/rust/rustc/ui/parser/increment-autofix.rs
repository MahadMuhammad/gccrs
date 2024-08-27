//@ run-rustfix

pub fn pre_regular() {
    let mut i = 0;
    ++i; // { dg-error "" "" { target *-*-* } }
    println!("{}", i);
}

pub fn pre_while() {
    let mut i = 0;
    while ++i < 5 {
// { dg-error "" "" { target *-*-* } .-1 }
        println!("{}", i);
    }
}

pub fn pre_regular_tmp() {
    let mut tmp = 0;
    ++tmp; // { dg-error "" "" { target *-*-* } }
    println!("{}", tmp);
}

pub fn pre_while_tmp() {
    let mut tmp = 0;
    while ++tmp < 5 {
// { dg-error "" "" { target *-*-* } .-1 }
        println!("{}", tmp);
    }
}

fn main() {}

