#![allow(unused_unsafe)]
#![allow(dead_code)]
#![deny(unsafe_code)]

struct Bar;
struct Bar2;
struct Bar3;

#[allow(unsafe_code)]
mod allowed_unsafe {
    fn allowed() { unsafe {} }
    unsafe fn also_allowed() {}
    unsafe trait AllowedUnsafe { }
    unsafe impl AllowedUnsafe for super::Bar {}
    #[no_mangle] fn allowed2() {}
    #[export_name = "foo"] fn allowed3() {}
}

macro_rules! unsafe_in_macro {
    () => {{
        #[no_mangle] fn foo() {} // { dg-error "" "" { target *-*-* } }
        #[no_mangle] static FOO: u32 = 5; // { dg-error "" "" { target *-*-* } }
        #[export_name = "bar"] fn bar() {}
// { dg-error "" "" { target *-*-* } .-1 }
        #[export_name = "BAR"] static BAR: u32 = 5;
// { dg-error "" "" { target *-*-* } .-1 }
        unsafe {} // { dg-error "" "" { target *-*-* } }
    }}
}

#[no_mangle] fn foo() {} // { dg-error "" "" { target *-*-* } }
#[no_mangle] static FOO: u32 = 5; // { dg-error "" "" { target *-*-* } }

trait AssocFnTrait {
    fn foo();
}

struct AssocFnFoo;

impl AssocFnFoo {
    #[no_mangle] fn foo() {} // { dg-error "" "" { target *-*-* } }
}

impl AssocFnTrait for AssocFnFoo {
    #[no_mangle] fn foo() {} // { dg-error "" "" { target *-*-* } }
}

#[export_name = "bar"] fn bar() {} // { dg-error "" "" { target *-*-* } }
#[export_name = "BAR"] static BAR: u32 = 5; // { dg-error "" "" { target *-*-* } }

#[link_section = ".example_section"] fn uwu() {} // { dg-error "" "" { target *-*-* } }
#[link_section = ".example_section"] static UWU: u32 = 5; // { dg-error "" "" { target *-*-* } }

struct AssocFnBar;

impl AssocFnBar {
    #[export_name = "bar"] fn bar() {} // { dg-error "" "" { target *-*-* } }
}

impl AssocFnTrait for AssocFnBar {
    #[export_name = "bar"] fn foo() {} // { dg-error "" "" { target *-*-* } }
}

unsafe fn baz() {} // { dg-error "" "" { target *-*-* } }
unsafe trait Foo {} // { dg-error "" "" { target *-*-* } }
unsafe impl Foo for Bar {} // { dg-error "" "" { target *-*-* } }

trait Baz {
    unsafe fn baz(&self); // { dg-error "" "" { target *-*-* } }
    unsafe fn provided(&self) {} // { dg-error "" "" { target *-*-* } }
    unsafe fn provided_override(&self) {} // { dg-error "" "" { target *-*-* } }
}

impl Baz for Bar {
    unsafe fn baz(&self) {} // { dg-error "" "" { target *-*-* } }
    unsafe fn provided_override(&self) {} // { dg-error "" "" { target *-*-* } }
}


#[allow(unsafe_code)]
trait A {
    unsafe fn allowed_unsafe(&self);
    unsafe fn allowed_unsafe_provided(&self) {}
}

#[allow(unsafe_code)]
impl Baz for Bar2 {
    unsafe fn baz(&self) {}
    unsafe fn provided_override(&self) {}
}

impl Baz for Bar3 {
    #[allow(unsafe_code)]
    unsafe fn baz(&self) {}
    unsafe fn provided_override(&self) {} // { dg-error "" "" { target *-*-* } }
}

#[allow(unsafe_code)]
unsafe trait B {
    fn dummy(&self) {}
}

trait C {
    #[allow(unsafe_code)]
    unsafe fn baz(&self);
    unsafe fn provided(&self) {} // { dg-error "" "" { target *-*-* } }
}

impl C for Bar {
    #[allow(unsafe_code)]
    unsafe fn baz(&self) {}
    unsafe fn provided(&self) {} // { dg-error "" "" { target *-*-* } }
}

impl C for Bar2 {
    unsafe fn baz(&self) {} // { dg-error "" "" { target *-*-* } }
}

trait D {
    #[allow(unsafe_code)]
    unsafe fn unsafe_provided(&self) {}
}

impl D for Bar {}

fn main() {
    unsafe {} // { dg-error "" "" { target *-*-* } }

    unsafe_in_macro!()
}

