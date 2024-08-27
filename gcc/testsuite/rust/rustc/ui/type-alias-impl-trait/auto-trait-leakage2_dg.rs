#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

mod m {
    use std::rc::Rc;

    type Foo = impl std::fmt::Debug; // { dg-note "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    pub fn foo() -> Foo {
        Rc::new(22_u32)
    }
}

fn is_send<T: Send>(_: T) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

fn main() {
    is_send(m::foo());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-note ".E0277." "" { target *-*-* } .-2 }
// { dg-note ".E0277." "" { target *-*-* } .-3 }
}

