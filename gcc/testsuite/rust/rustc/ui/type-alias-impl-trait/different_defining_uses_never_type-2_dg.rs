#![feature(type_alias_impl_trait)]

type Tait<'a> = impl Sized + 'a;

fn foo<'a, 'b>() -> Tait<'a> {
    if false {
        if { return } {
            let y: Tait<'b> = 1i32;
// { dg-error "" "" { target *-*-* } .-1 }
        }
    }
    let x: Tait<'a> = ();
    x
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

