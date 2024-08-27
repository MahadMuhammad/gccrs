#![derive(Clone, Copy)] // { dg-error "" "" { target *-*-* } }

use std::ptr::addr_of;

const UNINHABITED_VARIANT: () = unsafe {
    let v = *addr_of!(data).cast();
};

fn main() {}

