//@ check-pass
use std::cell::Cell;

pub enum JsValue {
    Undefined,
    Object(Cell<bool>),
}

impl ::std::ops::Drop for JsValue {
    fn drop(&mut self) {}
}

const UNDEFINED: &JsValue = &JsValue::Undefined;
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

fn main() {
}

