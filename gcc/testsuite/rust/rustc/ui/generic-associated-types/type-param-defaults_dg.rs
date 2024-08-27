// Check that we disallow GAT param defaults, even with `invalid_type_param_default` allowed

#![allow(invalid_type_param_default)]

trait Trait {
    type Assoc<T = u32>;
// { dg-error "" "" { target *-*-* } .-1 }
}

impl Trait for () {
    type Assoc<T = u32> = u64;
// { dg-error "" "" { target *-*-* } .-1 }
}

impl Trait for u32 {
    type Assoc<T = u32> = T;
// { dg-error "" "" { target *-*-* } .-1 }
}

trait Other {}
impl Other for u32 {}

fn foo<T>()
where
    T: Trait<Assoc = u32>,
    T::Assoc: Other {
    }

fn main() {
    // errors
    foo::<()>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
    // works
    foo::<u32>();
}

