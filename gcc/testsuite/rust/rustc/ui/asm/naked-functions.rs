//@ needs-asm-support
//@ ignore-nvptx64
//@ ignore-spirv

#![feature(naked_functions)]
#![feature(asm_unwind, linkage)]
#![crate_type = "lib"]

use std::arch::asm;

#[repr(C)]
pub struct P {
    x: u8,
    y: u16,
}

#[naked]
pub unsafe extern "C" fn patterns(
    mut a: u32,
// { dg-error "" "" { target *-*-* } .-1 }
    &b: &i32,
// { dg-error "" "" { target *-*-* } .-1 }
    (None | Some(_)): Option<std::ptr::NonNull<u8>>,
// { dg-error "" "" { target *-*-* } .-1 }
    P { x, y }: P,
// { dg-error "" "" { target *-*-* } .-1 }
) {
    asm!("", options(noreturn))
}

#[naked]
pub unsafe extern "C" fn inc(a: u32) -> u32 {
// { dg-error ".E0787." "" { target *-*-* } .-1 }
    a + 1
// { dg-error "" "" { target *-*-* } .-1 }
}

#[naked]
#[allow(asm_sub_register)]
pub unsafe extern "C" fn inc_asm(a: u32) -> u32 {
    asm!("/* {0} */", in(reg) a, options(noreturn));
// { dg-error ".E0787." "" { target *-*-* } .-1 }
// { dg-error ".E0787." "" { target *-*-* } .-2 }
}

#[naked]
pub unsafe extern "C" fn inc_closure(a: u32) -> u32 {
// { dg-error ".E0787." "" { target *-*-* } .-1 }
    (|| a + 1)()
}

#[naked]
pub unsafe extern "C" fn unsupported_operands() {
// { dg-error ".E0787." "" { target *-*-* } .-1 }
    let mut a = 0usize;
    let mut b = 0usize;
    let mut c = 0usize;
    let mut d = 0usize;
    let mut e = 0usize;
    const F: usize = 0usize;
    static G: usize = 0usize;
    asm!("/* {0} {1} {2} {3} {4} {5} {6} */",
// { dg-error ".E0787." "" { target *-*-* } .-1 }
         in(reg) a,
// { dg-error ".E0787." "" { target *-*-* } .-1 }
         inlateout(reg) b,
         inout(reg) c,
         lateout(reg) d,
         out(reg) e,
         const F,
         sym G,
    );
}

#[naked]
pub extern "C" fn missing_assembly() {
// { dg-error ".E0787." "" { target *-*-* } .-1 }
}

#[naked]
pub extern "C" fn too_many_asm_blocks() {
// { dg-error ".E0787." "" { target *-*-* } .-1 }
    unsafe {
        asm!("");
// { dg-error ".E0787." "" { target *-*-* } .-1 }
        asm!("");
// { dg-error ".E0787." "" { target *-*-* } .-1 }
        asm!("");
// { dg-error ".E0787." "" { target *-*-* } .-1 }
        asm!("", options(noreturn));
    }
}

pub fn outer(x: u32) -> extern "C" fn(usize) -> usize {
    #[naked]
    pub extern "C" fn inner(y: usize) -> usize {
// { dg-error ".E0787." "" { target *-*-* } .-1 }
        *&y
// { dg-error "" "" { target *-*-* } .-1 }
    }
    inner
}

#[naked]
unsafe extern "C" fn invalid_options() {
    asm!("", options(nomem, preserves_flags, noreturn));
// { dg-error ".E0787." "" { target *-*-* } .-1 }
}

#[naked]
unsafe extern "C" fn invalid_options_continued() {
    asm!("", options(readonly, nostack), options(pure));
// { dg-error ".E0787." "" { target *-*-* } .-1 }
// { dg-error ".E0787." "" { target *-*-* } .-2 }
// { dg-error ".E0787." "" { target *-*-* } .-3 }
}

#[naked]
unsafe extern "C" fn invalid_may_unwind() {
    asm!("", options(noreturn, may_unwind));
// { dg-error ".E0787." "" { target *-*-* } .-1 }
}

#[naked]
pub unsafe fn default_abi() {
// { dg-warning "" "" { target *-*-* } .-1 }
    asm!("", options(noreturn));
}

#[naked]
pub unsafe fn rust_abi() {
// { dg-warning "" "" { target *-*-* } .-1 }
    asm!("", options(noreturn));
}

#[naked]
pub extern "C" fn valid_a<T>() -> T {
    unsafe {
        asm!("", options(noreturn));
    }
}

#[naked]
pub extern "C" fn valid_b() {
    unsafe {
        {
            {
                asm!("", options(noreturn));
            };
        };
    }
}

#[naked]
pub unsafe extern "C" fn valid_c() {
    asm!("", options(noreturn));
}

#[cfg(target_arch = "x86_64")]
#[naked]
pub unsafe extern "C" fn valid_att_syntax() {
    asm!("", options(noreturn, att_syntax));
}

#[naked]
#[naked]
pub unsafe extern "C" fn allow_compile_error(a: u32) -> u32 {
    compile_error!("this is a user specified error")
// { dg-error "" "" { target *-*-* } .-1 }
}

#[naked]
pub unsafe extern "C" fn allow_compile_error_and_asm(a: u32) -> u32 {
    compile_error!("this is a user specified error");
// { dg-error "" "" { target *-*-* } .-1 }
    asm!("", options(noreturn))
}

#[naked]
pub unsafe extern "C" fn invalid_asm_syntax(a: u32) -> u32 {
    asm!(invalid_syntax)
// { dg-error "" "" { target *-*-* } .-1 }
}

#[cfg(target_arch = "x86_64")]
#[cfg_attr(target_pointer_width = "64", no_mangle)]
#[naked]
pub unsafe extern "C" fn compatible_cfg_attributes() {
    asm!("", options(noreturn, att_syntax));
}

#[allow(dead_code)]
#[warn(dead_code)]
#[deny(dead_code)]
#[forbid(dead_code)]
#[naked]
pub unsafe extern "C" fn compatible_diagnostic_attributes() {
    asm!("", options(noreturn, raw));
}

#[deprecated = "test"]
#[naked]
pub unsafe extern "C" fn compatible_deprecated_attributes() {
    asm!("", options(noreturn, raw));
}

#[cfg(target_arch = "x86_64")]
#[must_use]
#[naked]
pub unsafe extern "C" fn compatible_must_use_attributes() -> u64 {
    asm!(
        "
        mov rax, 42
        ret
        ",
        options(noreturn)
    )
}

#[export_name = "exported_function_name"]
#[link_section = ".custom_section"]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn compatible_ffi_attributes_1() {
    asm!("", options(noreturn, raw));
}

#[cold]
#[naked]
pub unsafe extern "C" fn compatible_codegen_attributes() {
    asm!("", options(noreturn, raw));
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[naked]
pub unsafe extern "C" fn compatible_target_feature() {
    asm!("", options(noreturn));
}

#[doc = "foo bar baz"]
/// a doc comment
// a normal comment
#[doc(alias = "ADocAlias")]
#[naked]
pub unsafe extern "C" fn compatible_doc_attributes() {
    asm!("", options(noreturn, raw));
}

#[linkage = "external"]
#[naked]
pub unsafe extern "C" fn compatible_linkage() {
    asm!("", options(noreturn, raw));
}

