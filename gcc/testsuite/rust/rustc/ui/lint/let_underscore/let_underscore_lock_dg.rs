//@ check-fail
use std::sync::{Arc, Mutex};

struct Struct<T> {
    a: T,
}

fn main() {
    let data = Arc::new(Mutex::new(0));
    let _ = data.lock().unwrap(); // { dg-error "" "" { target *-*-* } }

    let _ = data.lock(); // { dg-error "" "" { target *-*-* } }

    let (_, _) = (data.lock(), 1); // { dg-error "" "" { target *-*-* } }

    let (_a, Struct { a: _ }) = (0, Struct { a: data.lock() }); // { dg-error "" "" { target *-*-* } }

    (_ , _) = (data.lock(), 1); // { dg-error "" "" { target *-*-* } }

    let _b;
    (_b, Struct { a: _ }) = (0, Struct { a: data.lock() }); // { dg-error "" "" { target *-*-* } }
}

