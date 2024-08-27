//@ check-pass
#![warn(redundant_imports)]


use std::option::Option::Some;// { dg-warning "" "" { target *-*-* } }
use std::option::Option::None; // { dg-warning "" "" { target *-*-* } }

use std::result::Result::Ok;// { dg-warning "" "" { target *-*-* } }
use std::result::Result::Err;// { dg-warning "" "" { target *-*-* } }
use std::convert::{TryFrom, TryInto};

fn main() {
    let _a: Option<i32> = Some(1);
    let _b: Option<i32>  = None;
    let _c: Result<i32, String> = Ok(1);
    let _d: Result<i32, &str> = Err("error");
    let _e: Result<i32, _> = 8u8.try_into();
    let _f: Result<i32, _> = i32::try_from(8u8);
}

