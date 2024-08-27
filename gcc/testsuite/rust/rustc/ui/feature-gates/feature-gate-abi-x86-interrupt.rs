//@ needs-llvm-components: x86
//@ compile-flags: --target=x86_64-unknown-linux-gnu --crate-type=rlib
#![no_core]
#![feature(no_core, lang_items)]
#[lang="sized"]
trait Sized { }

extern "x86-interrupt" fn f7() {} // { dg-error ".E0658." "" { target *-*-* } }
trait Tr {
    extern "x86-interrupt" fn m7(); // { dg-error ".E0658." "" { target *-*-* } }
    extern "x86-interrupt" fn dm7() {} // { dg-error ".E0658." "" { target *-*-* } }
}

struct S;

// Methods in trait impl
impl Tr for S {
    extern "x86-interrupt" fn m7() {} // { dg-error ".E0658." "" { target *-*-* } }
}

// Methods in inherent impl
impl S {
    extern "x86-interrupt" fn im7() {} // { dg-error ".E0658." "" { target *-*-* } }
}

type A7 = extern "x86-interrupt" fn(); // { dg-error ".E0658." "" { target *-*-* } }

extern "x86-interrupt" {} // { dg-error ".E0658." "" { target *-*-* } }

