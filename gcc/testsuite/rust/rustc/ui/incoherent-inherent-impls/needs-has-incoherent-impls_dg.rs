//@ aux-build:extern-crate.rs
#![feature(rustc_attrs)]
extern crate extern_crate;

impl extern_crate::StructWithAttr {
// { dg-error ".E0390." "" { target *-*-* } .-1 }
    fn foo() {}
}
impl extern_crate::StructWithAttr {
    #[rustc_allow_incoherent_impl]
    fn bar() {}
}
impl extern_crate::StructNoAttr {
// { dg-error ".E0390." "" { target *-*-* } .-1 }
    fn foo() {}
}
impl extern_crate::StructNoAttr {
// { dg-error ".E0390." "" { target *-*-* } .-1 }
    #[rustc_allow_incoherent_impl]
    fn bar() {}
}
impl extern_crate::EnumWithAttr {
// { dg-error ".E0390." "" { target *-*-* } .-1 }
    fn foo() {}
}
impl extern_crate::EnumWithAttr {
    #[rustc_allow_incoherent_impl]
    fn bar() {}
}
impl extern_crate::EnumNoAttr {
// { dg-error ".E0390." "" { target *-*-* } .-1 }
    fn foo() {}
}
impl extern_crate::EnumNoAttr {
// { dg-error ".E0390." "" { target *-*-* } .-1 }
    #[rustc_allow_incoherent_impl]
    fn bar() {}
}

fn main() {}

