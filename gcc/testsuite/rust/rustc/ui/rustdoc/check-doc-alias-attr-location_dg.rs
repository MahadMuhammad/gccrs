#![crate_type = "lib"]

pub struct Bar;
pub trait Foo {
    type X;
    fn foo(x: u32) -> Self::X;
}

#[doc(alias = "foo")] // { dg-error "" "" { target *-*-* } }
extern "C" {}

#[doc(alias = "bar")] // { dg-error "" "" { target *-*-* } }
impl Bar {
    #[doc(alias = "const")]
    const A: u32 = 0;
}

#[doc(alias = "foobar")] // { dg-error "" "" { target *-*-* } }
impl Foo for Bar {
    #[doc(alias = "assoc")] // { dg-error "" "" { target *-*-* } }
    type X = i32;
    fn foo(#[doc(alias = "qux")] _x: u32) -> Self::X {
// { dg-error "" "" { target *-*-* } .-1 }
        #[doc(alias = "stmt")] // { dg-error "" "" { target *-*-* } }
        let x = 0;
        #[doc(alias = "expr")] // { dg-error "" "" { target *-*-* } }
        match x {
            #[doc(alias = "arm")] // { dg-error "" "" { target *-*-* } }
            _ => 0
        }
    }
}

