//@ needs-asm-support
//@ ignore-nvptx64
//@ ignore-spirv

// Tests that the use of named labels in the `asm!` macro are linted against
// except for in `#[naked]` fns.
// Using a named label is incorrect as per the RFC because for most cases
// the compiler cannot ensure that inline asm is emitted exactly once per
// codegen unit (except for naked fns) and so the label could be duplicated
// which causes less readable LLVM errors and in the worst cases causes ICEs
// or segfaults based on system dependent behavior and codegen flags.

#![feature(naked_functions)]

use std::arch::{asm, global_asm};

#[no_mangle]
pub static FOO: usize = 42;

fn main() {
    unsafe {
        // Basic usage
        asm!("bar: nop"); // { dg-error "" "" { target *-*-* } }

        // No following asm
        asm!("abcd:"); // { dg-error "" "" { target *-*-* } }

        // Multiple labels on one line
        asm!("foo: bar1: nop");
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

        // Multiple lines
        asm!("foo1: nop", "nop"); // { dg-error "" "" { target *-*-* } }
        asm!("foo2: foo3: nop", "nop");
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        asm!("nop", "foo4: nop"); // { dg-error "" "" { target *-*-* } }
        asm!("foo5: nop", "foo6: nop");
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

        // Statement separator
        asm!("foo7: nop; foo8: nop");
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        asm!("foo9: nop; nop"); // { dg-error "" "" { target *-*-* } }
        asm!("nop; foo10: nop"); // { dg-error "" "" { target *-*-* } }

        // Escaped newline
        asm!("bar2: nop\n bar3: nop");
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        asm!("bar4: nop\n nop"); // { dg-error "" "" { target *-*-* } }
        asm!("nop\n bar5: nop"); // { dg-error "" "" { target *-*-* } }
        asm!("nop\n bar6: bar7: nop");
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

        // Raw strings
        asm!(
            r"
            blah2: nop
            blah3: nop
            "
        );
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-4 }

        asm!(
            r###"
            nop
            nop ; blah4: nop
            "###
        );
// { dg-error "" "" { target *-*-* } .-3 }

        // Non-labels
        // should not trigger lint, but may be invalid asm
        asm!("ab cd: nop");

        // `blah:` does not trigger because labels need to be at the start
        // of the statement, and there was already a non-label
        asm!("1bar: blah: nop");

        // Only `blah1:` should trigger
        asm!("blah1: 2bar: nop"); // { dg-error "" "" { target *-*-* } }

        // Duplicate labels
        asm!("def: def: nop");
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        asm!("def: nop\ndef: nop");
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        asm!("def: nop; def: nop");
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

        // Trying to break parsing
        asm!(":");
        asm!("\n:\n");
        asm!("::::");

        // 0x3A is a ':'
        asm!("fooo\u{003A} nop"); // { dg-error "" "" { target *-*-* } }
        asm!("foooo\x3A nop"); // { dg-error "" "" { target *-*-* } }

        // 0x0A is a newline
        asm!("fooooo:\u{000A} nop"); // { dg-error "" "" { target *-*-* } }
        asm!("foooooo:\x0A nop"); // { dg-error "" "" { target *-*-* } }

        // Intentionally breaking span finding
        // equivalent to "ABC: nop"
        asm!("\x41\x42\x43\x3A\x20\x6E\x6F\x70"); // { dg-error "" "" { target *-*-* } }

        // Non-label colons - should pass
        asm!("mov rax, qword ptr fs:[0]");

        // Comments
        asm!(
            r"
            ab: nop // ab: does foo
            // cd: nop
            "
        );
// { dg-error "" "" { target *-*-* } .-4 }

        // Tests usage of colons in non-label positions
        asm!(":lo12:FOO"); // this is apparently valid aarch64

        // is there an example that is valid x86 for this test?
        asm!(":bbb nop");

        // non-ascii characters are not allowed in labels, so should not trigger the lint
        asm!("Ù: nop");
        asm!("testÙ: nop");
        asm!("_Ù_: nop");

        // Format arguments should be conservatively assumed to be valid characters in labels
        // Would emit `test_rax:` or similar
        #[allow(asm_sub_register)]
        {
            asm!("test_{}: nop", in(reg) 10); // { dg-error "" "" { target *-*-* } }
        }
        asm!("test_{}: nop", const 10); // { dg-error "" "" { target *-*-* } }
        asm!("test_{}: nop", sym main); // { dg-error "" "" { target *-*-* } }
        asm!("{}_test: nop", const 10); // { dg-error "" "" { target *-*-* } }
        asm!("test_{}_test: nop", const 10); // { dg-error "" "" { target *-*-* } }
        asm!("{}: nop", const 10); // { dg-error "" "" { target *-*-* } }

        asm!("{uwu}: nop", uwu = const 10); // { dg-error "" "" { target *-*-* } }
        asm!("{0}: nop", const 10); // { dg-error "" "" { target *-*-* } }
        asm!("{1}: nop", "/* {0} */", const 10, const 20); // { dg-error "" "" { target *-*-* } }

        // Test include_str in asm
        asm!(include_str!("named-asm-labels.s"));
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }

        // Test allowing or warning on the lint instead
        #[allow(named_asm_labels)]
        {
            asm!("allowed: nop"); // Should not emit anything
        }

        #[warn(named_asm_labels)]
        {
            asm!("warned: nop"); // { dg-warning "" "" { target *-*-* } }
        }
    }
}

// Trigger on naked fns too, even though they can't be inlined, reusing a
// label or LTO can cause labels to break
#[naked]
pub extern "C" fn foo() -> i32 {
    unsafe { asm!(".Lfoo: mov rax, {}; ret;", "nop", const 1, options(noreturn)) }
// { dg-error "" "" { target *-*-* } .-1 }
}

// Make sure that non-naked attributes *do* still let the lint happen
#[no_mangle]
pub extern "C" fn bar() {
    unsafe { asm!(".Lbar: mov rax, {}; ret;", "nop", const 1, options(noreturn)) }
// { dg-error "" "" { target *-*-* } .-1 }
}

#[naked]
pub extern "C" fn aaa() {
    fn _local() {}

    unsafe { asm!(".Laaa: nop; ret;", options(noreturn)) } // { dg-error "" "" { target *-*-* } }
}

pub fn normal() {
    fn _local1() {}

    #[naked]
    pub extern "C" fn bbb() {
        fn _very_local() {}

        unsafe { asm!(".Lbbb: nop; ret;", options(noreturn)) } // { dg-error "" "" { target *-*-* } }
    }

    fn _local2() {}
}

// Make sure that the lint happens within closures
fn closures() {
    || unsafe {
        asm!("closure1: nop"); // { dg-error "" "" { target *-*-* } }
    };

    move || unsafe {
        asm!("closure2: nop"); // { dg-error "" "" { target *-*-* } }
    };

    || {
        #[naked]
        unsafe extern "C" fn _nested() {
            asm!("ret;", options(noreturn));
        }

        unsafe {
            asm!("closure3: nop"); // { dg-error "" "" { target *-*-* } }
        }
    };
}

// Don't trigger on global asm
global_asm!("aaaaaaaa: nop");

