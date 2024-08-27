//@ revisions: x86_64 aarch64

//@ [x86_64] compile-flags: --target x86_64-unknown-linux-gnu
//@ [aarch64] compile-flags: --target aarch64-unknown-linux-gnu

//@ [x86_64] needs-llvm-components: x86
//@ [aarch64] needs-llvm-components: aarch64

#![feature(no_core, lang_items, rustc_attrs)]
#![no_core]

#[rustc_builtin_macro]
macro_rules! asm {
    () => {};
}
#[rustc_builtin_macro]
macro_rules! global_asm {
    () => {};
}

#[lang = "sized"]
trait Sized {}

#[lang = "copy"]
trait Copy {}

fn main() {
    let mut foo = 0;
    unsafe {
        asm!("{}");
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{1}", in(reg) foo);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        asm!("{a}");
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", a = in(reg) foo);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        asm!("{1}", a = in(reg) foo);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        #[cfg(any(x86_64))]
        asm!("{}", in("eax") foo);
// { dg-error "" "" { target *-*-* } .-1 }
        #[cfg(any(aarch64))]
        asm!("{}", in("x0") foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{:foo}", in(reg) foo);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
        asm!("", in(reg) 0, in(reg) 1);
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

const FOO: i32 = 1;
global_asm!("{}");
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{1}", const FOO);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
global_asm!("{a}");
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{}", a = const FOO);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
global_asm!("{1}", a = const FOO);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
global_asm!("{:foo}", const FOO);
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("", const FOO, const FOO);
// { dg-error "" "" { target *-*-* } .-1 }

