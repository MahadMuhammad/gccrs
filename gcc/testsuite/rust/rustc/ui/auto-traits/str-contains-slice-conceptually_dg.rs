#![feature(negative_impls)]
#![feature(auto_traits)]

auto trait AutoTrait {}

impl<T> !AutoTrait for [T] {}

fn needs_auto_trait<T: AutoTrait + ?Sized>() {}

fn main() {
  needs_auto_trait::<str>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

