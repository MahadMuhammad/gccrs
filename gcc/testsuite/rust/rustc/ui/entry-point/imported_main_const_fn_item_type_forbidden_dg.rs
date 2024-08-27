#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]
pub mod foo {
    type MainFn = impl Fn();

    fn bar() {}
    pub const BAR: MainFn = bar;
}

use foo::BAR as main; // { dg-error ".E0601." "" { target *-*-* } }

