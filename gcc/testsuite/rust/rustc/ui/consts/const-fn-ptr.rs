const fn make_fn_ptr() -> fn() {
    || {}
}

static STAT: () = make_fn_ptr()();
// { dg-error "" "" { target *-*-* } .-1 }

const CONST: () = make_fn_ptr()();
// { dg-error "" "" { target *-*-* } .-1 }

const fn call_ptr() {
    make_fn_ptr()();
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

