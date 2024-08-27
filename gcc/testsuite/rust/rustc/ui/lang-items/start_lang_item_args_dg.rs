//@ check-fail
//@ revisions: missing_all_args missing_sigpipe_arg missing_ret start_ret too_many_args
//@ revisions: main_ty main_args main_ret argc argv_inner_ptr argv sigpipe

#![feature(lang_items, no_core)]
#![no_core]

#[lang = "copy"]
pub trait Copy {}
#[lang = "sized"]
pub trait Sized {}

#[cfg(missing_all_args)]
#[lang = "start"]
fn start<T>() -> isize {
// { dg-error "" "" { target *-*-* } .-1 }
    100
}

#[cfg(missing_sigpipe_arg)]
#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8) -> isize {
// { dg-error "" "" { target *-*-* } .-1 }
    100
}

#[cfg(missing_ret)]
#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8, _sigpipe: u8) {}
// { dg-error "" "" { target *-*-* } .-1 }

#[cfg(start_ret)]
#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8, _sigpipe: u8) -> u8 {
// { dg-error "" "" { target *-*-* } .-1 }
    100
}

#[cfg(too_many_args)]
#[lang = "start"]
fn start<T>(
// { dg-error "" "" { target *-*-* } .-1 }
    _main: fn() -> T,
    _argc: isize,
    _argv: *const *const u8,
    _sigpipe: u8,
    _extra_arg: (),
) -> isize {
    100
}

#[cfg(main_ty)]
#[lang = "start"]
fn start<T>(_main: u64, _argc: isize, _argv: *const *const u8, _sigpipe: u8) -> isize {
// { dg-error "" "" { target *-*-* } .-1 }
    100
}

#[cfg(main_args)]
#[lang = "start"]
fn start<T>(_main: fn(i32) -> T, _argc: isize, _argv: *const *const u8, _sigpipe: u8) -> isize {
// { dg-error "" "" { target *-*-* } .-1 }
    100
}

#[cfg(main_ret)]
#[lang = "start"]
fn start<T>(_main: fn() -> u16, _argc: isize, _argv: *const *const u8, _sigpipe: u8) -> isize {
// { dg-error "" "" { target *-*-* } .-1 }
    100
}

#[cfg(argc)]
#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: i8, _argv: *const *const u8, _sigpipe: u8) -> isize {
// { dg-error "" "" { target *-*-* } .-1 }
    100
}

#[cfg(argv_inner_ptr)]
#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const usize, _sigpipe: u8) -> isize {
// { dg-error "" "" { target *-*-* } .-1 }
    100
}

#[cfg(argv)]
#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: isize, _argv: u8, _sigpipe: u8) -> isize {
// { dg-error "" "" { target *-*-* } .-1 }
    100
}

#[cfg(sigpipe)]
#[lang = "start"]
fn start<T>(_main: fn() -> T, _argc: isize, _argv: *const *const u8, _sigpipe: i64) -> isize {
// { dg-error "" "" { target *-*-* } .-1 }
    100
}

fn main() {}

