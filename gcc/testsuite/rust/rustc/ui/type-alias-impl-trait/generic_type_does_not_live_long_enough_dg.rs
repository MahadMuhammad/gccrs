#![feature(type_alias_impl_trait)]

fn main() {
    let y = 42;
    let x = wrong_generic(&y);
    let z: i32 = x;
// { dg-error ".E0792." "" { target *-*-* } .-1 }

    type WrongGeneric<T> = impl 'static;
// { dg-error "" "" { target *-*-* } .-1 }

    fn wrong_generic<T>(t: T) -> WrongGeneric<T> {
        t
// { dg-error ".E0310." "" { target *-*-* } .-1 }
    }
}

