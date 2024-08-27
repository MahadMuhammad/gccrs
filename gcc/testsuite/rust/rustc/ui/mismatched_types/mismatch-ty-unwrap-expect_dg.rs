//@ run-rustfix
#![allow(unused, dead_code)]

fn func() -> Option<i32> {
    Some(1)
}

fn test_unwrap() -> Result<i32, ()> {
    let b: Result<i32, ()> = Ok(1);
    let v: i32 = b; // { dg-error ".E0308." "" { target *-*-* } }
    Ok(v)
}

fn test_unwrap_option() -> Option<i32> {
    let b = Some(1);
    let v: i32 = b; // { dg-error ".E0308." "" { target *-*-* } }
    Some(v)
}

fn main() {
    let a = Some(1);
    let v: i32 = a; // { dg-error ".E0308." "" { target *-*-* } }

    let b: Result<i32, ()> = Ok(1);
    let v: i32 = b; // { dg-error ".E0308." "" { target *-*-* } }

    let v: i32 = func(); // { dg-error ".E0308." "" { target *-*-* } }

    let a = None;
    let v: i32 = a; // { dg-error ".E0308." "" { target *-*-* } }
}

