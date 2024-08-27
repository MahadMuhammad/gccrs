#![deny(unnameable_types)]

mod m {
    pub struct PubStruct(pub i32); // { dg-error "" "" { target *-*-* } }

    pub enum PubE { // { dg-error "" "" { target *-*-* } }
        V(i32),
    }

    pub trait PubTr { // { dg-error "" "" { target *-*-* } }
        const C : i32 = 0;
        type Alias;
        fn f() {}
    }

    impl PubTr for PubStruct {
        type Alias = i32;
        fn f() {}
    }
}

pub trait Unnameable<T> {}

impl Unnameable<m::PubStruct> for i32 {}
impl Unnameable<m::PubE> for i32 {}
impl<T> Unnameable<T> for u32 where T: m::PubTr {}

fn main() {}

