//! This test used to ICE in typeck because of the type/const mismatch,
//! even though wfcheck already errored.
//! issue: rust-lang/rust#123457

pub struct KeyHolder<const K: u8> {}

pub trait ContainsKey<const K: u8> {}

pub trait SubsetExcept<P> {}

impl<K> ContainsKey<K> for KeyHolder<K> {}
// { dg-error ".E0747." "" { target *-*-* } .-1 }
// { dg-error ".E0747." "" { target *-*-* } .-2 }

impl<P, T: ContainsKey<0>> SubsetExcept<P> for T {}

pub fn remove_key<K, S: SubsetExcept<K>>() -> S {
    loop {}
}

fn main() {
    let map: KeyHolder<0> = remove_key::<_, _>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

