#![allow(unused)]

fn a(i: i32) -> i32 {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }

fn b(opt_str: Option<String>) {
    let s: String = if let Some(s) = opt_str {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    } else {
        String::new()
    };
}

fn c() -> Option<i32> {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let x = Some(1);
}

fn d(opt_str: Option<String>) {
    let s: String = if let Some(s) = opt_str {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    } else {
        String::new()
    };
}

fn d2(opt_str: Option<String>) {
    let s = if let Some(s) = opt_str {
    } else {
        String::new()
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    };
}

fn e(opt_str: Option<String>) {
    let s: String = match opt_str {
        Some(s) => {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        None => String::new(),
    };
}

fn e2(opt_str: Option<String>) {
    let s = match opt_str {
        Some(s) => {}
        None => String::new(),
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    };
}

fn main() {}

