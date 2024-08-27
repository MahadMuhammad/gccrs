mod a {
    mod b {
        mod c {
            pub struct E;
        }

        mod d {
            #[derive(Debug)]
            pub struct E;
        }

        pub use self::d::*;
        pub use self::c::*;
    }

    pub use self::b::*;
}

use self::a::E::in_exist;
// { dg-error ".E0432." "" { target *-*-* } .-1 }
// { dg-warning ".E0432." "" { target *-*-* } .-2 }
// { dg-warning ".E0432." "" { target *-*-* } .-3 }

fn main() {}

