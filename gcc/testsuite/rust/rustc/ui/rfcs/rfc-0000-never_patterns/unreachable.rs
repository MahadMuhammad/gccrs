#![feature(never_patterns)]
#![allow(incomplete_features)]
#![allow(dead_code, unreachable_code)]
#![deny(unreachable_patterns)]

#[derive(Copy, Clone)]
enum Void {}

fn main() {
    let res_void: Result<bool, Void> = Ok(true);

    match res_void {
        Ok(_x) => {}
        Err(!),
// { dg-error "" "" { target *-*-* } .-1 }
    }
    let (Ok(_x) | Err(!)) = res_void;
// { dg-error "" "" { target *-*-* } .-1 }
    if let Err(!) = res_void {}
// { dg-error "" "" { target *-*-* } .-1 }
    if let (Ok(true) | Err(!)) = res_void {}
// { dg-error "" "" { target *-*-* } .-1 }
    for (Ok(mut _x) | Err(!)) in [res_void] {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn foo((Ok(_x) | Err(!)): Result<bool, Void>) {}
// { dg-error "" "" { target *-*-* } .-1 }

