// ICE: ImmTy { imm: Scalar(alloc1), ty: *const dyn Sync } input to a fat-to-thin cast (*const dyn Sync -> *const usize
// or with -Zextra-const-ub-checks: expected wide pointer extra data (e.g. slice length or trait object vtable)
// issue: rust-lang/rust#121413
//@ compile-flags: -Zextra-const-ub-checks
// ignore-tidy-linelength
#![feature(const_refs_to_static)]
const REF_INTERIOR_MUT: &usize = {
// { help "" "" { target *-*-* } .-1 }
    static FOO: Sync = AtomicUsize::new(0);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-warning ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
// { dg-error ".E0277." "" { target *-*-* } .-4 }
// { dg-warning ".E0277." "" { target *-*-* } .-5 }
// { help ".E0277." "" { target *-*-* } .-6 }
// { help ".E0277." "" { target *-*-* } .-7 }
// { help ".E0277." "" { target *-*-* } .-8 }
    unsafe { &*(&FOO as *const _ as *const usize) }
};
pub fn main() {}

