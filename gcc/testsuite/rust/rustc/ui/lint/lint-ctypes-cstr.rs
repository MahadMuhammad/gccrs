#![crate_type = "lib"]
#![deny(improper_ctypes, improper_ctypes_definitions)]

use std::ffi::{CStr, CString};

extern "C" {
    fn take_cstr(s: CStr);
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    fn take_cstr_ref(s: &CStr);
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    fn take_cstring(s: CString);
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    fn take_cstring_ref(s: &CString);
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    fn no_special_help_for_mut_cstring(s: *mut CString);
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    fn no_special_help_for_mut_cstring_ref(s: &mut CString);
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

extern "C" fn rust_take_cstr_ref(s: &CStr) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
extern "C" fn rust_take_cstring(s: CString) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
extern "C" fn rust_no_special_help_for_mut_cstring(s: *mut CString) {}
extern "C" fn rust_no_special_help_for_mut_cstring_ref(s: &mut CString) {}

