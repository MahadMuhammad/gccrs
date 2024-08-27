// ICE !base.layout().is_sized()
// issue: rust-lang/rust#123078

struct S {
    a: [u8],
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    b: (),
}

const C: S = unsafe { std::mem::transmute(()) };
// { dg-error ".E0512." "" { target *-*-* } .-1 }
const _: [(); {
    C;
    0
}] = [];

pub fn main() {}

