#![feature(type_alias_impl_trait)]

pub type Opaque<'a> = impl Sized;

fn get_one<'a>(a: *mut &'a str) -> impl IntoIterator<Item = Opaque<'a>> {
    if a.is_null() {
        Some(a)
    } else {
        None::<Opaque<'static>>
// { dg-error ".E0700." "" { target *-*-* } .-1 }
    }
}

fn main() {}

