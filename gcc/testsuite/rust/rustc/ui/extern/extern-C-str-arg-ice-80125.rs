// issue: rust-lang/rust#80125
//@ check-pass
type ExternCallback = extern "C" fn(*const u8, u32, str);
// { dg-warning "" "" { target *-*-* } .-1 }

pub struct Struct(ExternCallback);

#[no_mangle]
pub extern "C" fn register_something(bind: ExternCallback) -> Struct {
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    Struct(bind)
}

fn main() {}

