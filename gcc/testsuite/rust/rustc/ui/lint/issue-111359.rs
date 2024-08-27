#[deny(missing_debug_implementations)]
#[deny(missing_copy_implementations)]

mod priv_mod {
    use std::convert::TryFrom;

    pub struct BarPub;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    struct BarPriv;

    impl<'a> TryFrom<BarPriv> for u8 {
        type Error = ();
        fn try_from(o: BarPriv) -> Result<Self, ()> {
            unimplemented!()
        }
    }

    impl<'a> TryFrom<BarPub> for u8 {
        type Error = ();
        fn try_from(o: BarPub) -> Result<Self, ()> {
            unimplemented!()
        }
    }
}

fn main() {}

