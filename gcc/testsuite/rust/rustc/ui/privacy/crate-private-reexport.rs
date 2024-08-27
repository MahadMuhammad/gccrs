fn f1() {}
enum E1 { V }
struct S1 {
    #[rustfmt::skip]
    bar: i32,
}
mod m1 {
    pub use ::f1; // { dg-error ".E0364." "" { target *-*-* } }
    pub use ::S1; // { dg-error ".E0365." "" { target *-*-* } }
    pub use ::E1; // { dg-error ".E0365." "" { target *-*-* } }
    pub use ::E1::V; // { dg-error ".E0364." "" { target *-*-* } }
}

pub(crate) fn f2() {}
pub(crate) enum E2 {
    V
}
pub(crate) struct S2 {
    #[rustfmt::skip]
    bar: i32,
}
mod m2 {
    pub use ::f2; // { dg-error ".E0364." "" { target *-*-* } }
    pub use ::S2; // { dg-error ".E0365." "" { target *-*-* } }
    pub use ::E2; // { dg-error ".E0365." "" { target *-*-* } }
    pub use ::E2::V; // { dg-error ".E0364." "" { target *-*-* } }
}

mod m3 {
    pub(crate) fn f3() {}
    pub(crate) enum E3 {
        V
    }
    pub(crate) struct S3 {
        #[rustfmt::skip]
        bar: i32,
    }
}
pub use m3::f3; // { dg-error ".E0364." "" { target *-*-* } }
pub use m3::S3; // { dg-error ".E0365." "" { target *-*-* } }
pub use m3::E3; // { dg-error ".E0365." "" { target *-*-* } }
pub use m3::E3::V; // { dg-error ".E0364." "" { target *-*-* } }

pub(self) fn f4() {}
pub use ::f4 as f5; // { dg-error ".E0364." "" { target *-*-* } }

pub mod m10 {
    pub mod m {
        pub(super) fn f6() {}
        pub(crate) fn f7() {}
        pub(in crate::m10) fn f8() {}
    }
    pub use self::m::f6; // { dg-error ".E0364." "" { target *-*-* } }
    pub use self::m::f7; // { dg-error ".E0364." "" { target *-*-* } }
    pub use self::m::f8; // { dg-error ".E0364." "" { target *-*-* } }
}
pub use m10::m::f6; // { dg-error ".E0603." "" { target *-*-* } }
pub use m10::m::f7; // { dg-error ".E0364." "" { target *-*-* } }
pub use m10::m::f8; // { dg-error ".E0603." "" { target *-*-* } }

pub mod m11 {
    pub(self) fn f9() {}
}
pub use m11::f9; // { dg-error ".E0603." "" { target *-*-* } }

fn main() {}

