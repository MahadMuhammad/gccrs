extern "C" unsafe {
// { dg-error "" "" { target *-*-* } .-1 }
    unsafe fn foo();
}

fn main() {}

