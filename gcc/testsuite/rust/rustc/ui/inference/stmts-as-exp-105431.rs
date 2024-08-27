#![allow(unused)]

fn test_if() -> i32 {
    let x = if true {
        eprintln!("hello");
        3;
    }
    else {
        4;
    };
    x // { dg-error ".E0308." "" { target *-*-* } }
}

fn test_if_without_binding() -> i32 {
    if true { // { dg-error ".E0308." "" { target *-*-* } }
        eprintln!("hello");
        3;
    }
    else { // { dg-error ".E0308." "" { target *-*-* } }
        4;
    }
}

fn test_match() -> i32 {
    let v = 1;
    let res = match v {
        1 => { 1; }
        _ => { 2; }
    };
    res // { dg-error ".E0308." "" { target *-*-* } }
}

fn test_match_match_without_binding() -> i32 {
    let v = 1;
    match v {
        1 => { 1; } // { dg-error ".E0308." "" { target *-*-* } }
        _ => { 2; } // { dg-error ".E0308." "" { target *-*-* } }
    }
}

fn test_match_arm_different_types() -> i32 {
    let v = 1;
    let res = match v {
        1 => { if 1 < 2 { 1 } else { 2 } }
        _ => { 2; } // { dg-error ".E0308." "" { target *-*-* } }
    };
    res
}

fn test_if_match_mixed() -> i32 {
    let x = if true {
        3;
    } else {
        match 1 {
            1 => { 1 }
            _ => { 2 }
        };
    };
    x // { dg-error ".E0308." "" { target *-*-* } }
}

fn test_if_match_mixed_failed() -> i32 {
    let x = if true {
        3;
    } else {
        // because this is a tailed expr, so we won't check deeper
        match 1 {
            1 => { 33; }
            _ => { 44; }
        }
    };
    x // { dg-error ".E0308." "" { target *-*-* } }
}


fn main() {}

