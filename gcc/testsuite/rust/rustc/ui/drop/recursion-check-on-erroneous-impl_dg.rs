// can't use build-fail, because this also fails check-fail, but
// the ICE from #120787 only reproduces on build-fail.
//@ compile-flags: --emit=mir

struct PrintOnDrop<'a>(&'a str);

impl Drop for PrintOnDrop<'_> {
    fn drop() {} // { dg-error ".E0186." "" { target *-*-* } }
}

fn main() {}

