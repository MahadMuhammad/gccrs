//@ run-rustfix
#![allow(dead_code)]

fn duplicate_t<T>(t: T) -> (T, T) {
// { help "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    (t, t) // { dg-error ".E0382." "" { target *-*-* } }
}

fn duplicate_opt<T>(t: Option<T>) -> (Option<T>, Option<T>) {
// { help "" "" { target *-*-* } .-1 }
    (t, t) // { dg-error ".E0382." "" { target *-*-* } }
}

fn duplicate_tup1<T>(t: (T,)) -> ((T,), (T,)) {
// { help "" "" { target *-*-* } .-1 }
    (t, t) // { dg-error ".E0382." "" { target *-*-* } }
}

fn duplicate_tup2<A, B>(t: (A, B)) -> ((A, B), (A, B)) {
// { help "" "" { target *-*-* } .-1 }
    (t, t) // { dg-error ".E0382." "" { target *-*-* } }
}

fn duplicate_custom<T>(t: S<T>) -> (S<T>, S<T>) {
// { help "" "" { target *-*-* } .-1 }
    (t, t) // { dg-error ".E0382." "" { target *-*-* } }
}

struct S<T>(T);
trait Trait {}
impl<T: Trait + Clone> Clone for S<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T: Trait + Copy> Copy for S<T> {}

trait A {}
trait B {}

// Test where bounds are added with different bound placements
fn duplicate_custom_1<T>(t: S<T>) -> (S<T>, S<T>) where {
// { help "" "" { target *-*-* } .-1 }
    (t, t) // { dg-error ".E0382." "" { target *-*-* } }
}

fn duplicate_custom_2<T>(t: S<T>) -> (S<T>, S<T>)
where
    T: A,
// { help "" "" { target *-*-* } .-1 }
{
    (t, t) // { dg-error ".E0382." "" { target *-*-* } }
}

fn duplicate_custom_3<T>(t: S<T>) -> (S<T>, S<T>)
where
    T: A,
// { help "" "" { target *-*-* } .-1 }
    T: B,
{
    (t, t) // { dg-error ".E0382." "" { target *-*-* } }
}

fn duplicate_custom_4<T: A>(t: S<T>) -> (S<T>, S<T>)
// { help "" "" { target *-*-* } .-1 }
where
    T: B,
{
    (t, t) // { dg-error ".E0382." "" { target *-*-* } }
}

#[rustfmt::skip]
fn existing_colon<T:>(t: T) {
// { help "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    [t, t]; // { dg-error ".E0382." "" { target *-*-* } }
}

fn existing_colon_in_where<T>(t: T) // { help "" "" { target *-*-* } }
where
    T:,
// { help "" "" { target *-*-* } .-1 }
{
    [t, t]; // { dg-error ".E0382." "" { target *-*-* } }
}

fn main() {}

