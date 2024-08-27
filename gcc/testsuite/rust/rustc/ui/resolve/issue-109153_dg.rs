use foo::*;

mod foo {
    pub mod bar {
        pub mod bar {
            pub mod bar {}
        }
    }
}

use bar::bar; // { dg-error ".E0659." "" { target *-*-* } }
use bar::*;

fn main() { }

