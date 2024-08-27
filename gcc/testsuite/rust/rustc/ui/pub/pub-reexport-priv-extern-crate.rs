extern crate core;
pub use core as reexported_core; // { dg-error ".E0365." "" { target *-*-* } }
// { dg-warning ".E0365." "" { target *-*-* } .-1 }

mod foo1 {
    extern crate core;
}

mod foo2 {
    use foo1::core; // { dg-error ".E0603." "" { target *-*-* } }
    pub mod bar {
        extern crate core;
    }
}

mod baz {
    pub use foo2::bar::core; // { dg-error ".E0603." "" { target *-*-* } }
}

fn main() {}

