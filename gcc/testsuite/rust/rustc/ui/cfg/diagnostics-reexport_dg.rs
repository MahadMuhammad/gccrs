pub mod inner {
    #[cfg(FALSE)]
    mod gone {
        pub fn uwu() {}
    }

    #[cfg(FALSE)] // { dg-note "" "" { target *-*-* } }
    pub use super::uwu;
// { dg-note "" "" { target *-*-* } .-1 }
}

pub use a::x;
// { dg-error ".E0432." "" { target *-*-* } .-1 }
// { dg-note ".E0432." "" { target *-*-* } .-2 }

mod a {
    #[cfg(FALSE)] // { dg-note "" "" { target *-*-* } }
    pub fn x() {}
// { dg-note "" "" { target *-*-* } .-1 }
}

pub use b::{x, y};
// { dg-error ".E0432." "" { target *-*-* } .-1 }
// { dg-note ".E0432." "" { target *-*-* } .-2 }
// { dg-note ".E0432." "" { target *-*-* } .-3 }

mod b {
    #[cfg(FALSE)] // { dg-note "" "" { target *-*-* } }
    pub fn x() {}
// { dg-note "" "" { target *-*-* } .-1 }
    #[cfg(FALSE)] // { dg-note "" "" { target *-*-* } }
    pub fn y() {}
// { dg-note "" "" { target *-*-* } .-1 }
}

fn main() {
    // There is no uwu at this path - no diagnostic.
    inner::uwu(); // { dg-error ".E0425." "" { target *-*-* } }
// { dg-note ".E0425." "" { target *-*-* } .-1 }
}

