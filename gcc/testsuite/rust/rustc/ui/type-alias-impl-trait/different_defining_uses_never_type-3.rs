#![feature(type_alias_impl_trait)]

type Tait<T> = impl Sized;

fn foo<T, U>() -> Tait<T> {
    if false {
        if { return } {
            let y: Tait<U> = 1i32;
// { dg-error "" "" { target *-*-* } .-1 }
        }
    }
    let x: Tait<T> = ();
    x
}

fn main() {}

