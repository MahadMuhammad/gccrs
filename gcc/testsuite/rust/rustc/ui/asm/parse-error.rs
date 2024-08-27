//@ needs-asm-support

use std::arch::{asm, global_asm};

fn main() {
    let mut foo = 0;
    let mut bar = 0;
    unsafe {
        asm!();
// { dg-error "" "" { target *-*-* } .-1 }
        asm!(foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}" foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", in foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", in(reg foo));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", in(reg));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", inout(=) foo => bar);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", inout(reg) foo =>);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", in(reg) foo => bar);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", sym foo + bar);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", options(foo));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", options(nomem foo));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", options(nomem, foo));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", options(), const foo);
// { dg-error ".E0435." "" { target *-*-* } .-1 }

        // test that asm!'s clobber_abi doesn't accept non-string literals
        // see also https://github.com/rust-lang/rust/issues/112635
        asm!("", clobber_abi());
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi(foo));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi("C" foo));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi("C", foo));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi(1));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi(()));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi(uwu));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi({}));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi(loop {}));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi(if));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi(do));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi(<));
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("", clobber_abi(.));
// { dg-error "" "" { target *-*-* } .-1 }

        asm!("{}", clobber_abi("C"), const foo);
// { dg-error ".E0435." "" { target *-*-* } .-1 }
        asm!("", options(), clobber_abi("C"));
        asm!("{}", options(), clobber_abi("C"), const foo);
// { dg-error ".E0435." "" { target *-*-* } .-1 }
        asm!("{a}", a = const foo, a = const bar);
// { dg-error ".E0435." "" { target *-*-* } .-1 }
// { dg-error ".E0435." "" { target *-*-* } .-2 }
// { dg-error ".E0435." "" { target *-*-* } .-3 }
// { dg-error ".E0435." "" { target *-*-* } .-4 }

        asm!("", options(), "");
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", in(reg) foo, "{}", out(reg) foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!(format!("{{{}}}", 0), in(reg) foo);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{1}", format!("{{{}}}", 0), in(reg) foo, out(reg) bar);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", in(reg) _);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", inout(reg) _);
// { dg-error "" "" { target *-*-* } .-1 }
        asm!("{}", inlateout(reg) _);
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

const FOO: i32 = 1;
const BAR: i32 = 2;
global_asm!();
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!(FOO);
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{}" FOO);
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{}", FOO);
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{}", const);
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{}", const(reg) FOO);
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("", options(FOO));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("", options(FOO,));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("", options(nomem FOO));
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
global_asm!("", options(nomem, FOO));
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
global_asm!("{}", options(), const FOO);
global_asm!("", clobber_abi(FOO));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("", clobber_abi("C" FOO));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("", clobber_abi("C", FOO));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{}", clobber_abi("C"), const FOO);
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("", options(), clobber_abi("C"));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{}", options(), clobber_abi("C"), const FOO);
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("", clobber_abi("C"), clobber_abi("C"));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{a}", a = const FOO, a = const BAR);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
global_asm!("", options(), "");
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{}", const FOO, "{}", const FOO);
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!(format!("{{{}}}", 0), const FOO);
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{1}", format!("{{{}}}", 0), const FOO, const BAR);
// { dg-error "" "" { target *-*-* } .-1 }

global_asm!("{}", in(reg));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{}", out(reg));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{}", lateout(reg));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{}", inout(reg));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{}", inlateout(reg));
// { dg-error "" "" { target *-*-* } .-1 }
global_asm!("{}", label(reg));
// { dg-error "" "" { target *-*-* } .-1 }

