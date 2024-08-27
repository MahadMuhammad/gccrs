// test for ICE when casting extern "C" fn when it has a non-FFI-safe argument
// issue: rust-lang/rust#52334
//@ check-pass
//@ normalize-stderr-test: "\[i8\]" -> "[i8 or u8 (arch dependant)]"
//@ normalize-stderr-test: "\[u8\]" -> "[i8 or u8 (arch dependant)]"

type Foo = extern "C" fn(::std::ffi::CStr);
// { dg-warning "" "" { target *-*-* } .-1 }
extern "C" {
    fn meh(blah: Foo);
// { dg-warning "" "" { target *-*-* } .-1 }
}

fn main() {
    meh as usize;
}

