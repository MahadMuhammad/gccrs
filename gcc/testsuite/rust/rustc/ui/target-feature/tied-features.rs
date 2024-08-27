//@ build-fail
//@ compile-flags: --crate-type=rlib --target=aarch64-unknown-linux-gnu
//@ needs-llvm-components: aarch64
#![feature(no_core, lang_items)]
#![no_core]

#[lang="sized"]
trait Sized {}

// FIXME: this should not need to be public.
pub fn main() {
    #[target_feature(enable = "pacg")]
// { dg-error "" "" { target *-*-* } .-1 }
    unsafe fn inner() {}

    unsafe {
        foo();
        bar();
        baz();
        inner();
    }
}

#[target_feature(enable = "paca")]
// { dg-error "" "" { target *-*-* } .-1 }
unsafe fn foo() {}


#[target_feature(enable = "paca,pacg")]
unsafe fn bar() {}

#[target_feature(enable = "paca")]
#[target_feature(enable = "pacg")]
unsafe fn baz() {}

