#![feature(decl_macro)]

trait Trait {
    fn foo() {}
}

macro trait_impl() {
    fn foo() {}
}

// Check that we error on multiple impl items that resolve to the same trait item.
impl Trait for i32 {
    trait_impl!();
    fn foo() {}
// { dg-error ".E0201." "" { target *-*-* } .-1 }
}

struct Type;

// Check that we do not error with inherent impls.
impl Type {
    trait_impl!();
    fn foo() {}
}

fn main() {}

