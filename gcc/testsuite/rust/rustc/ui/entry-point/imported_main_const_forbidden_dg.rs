pub mod foo {
    pub const BAR: usize = 42;
}

use foo::BAR as main; // { dg-error ".E0601." "" { target *-*-* } }

