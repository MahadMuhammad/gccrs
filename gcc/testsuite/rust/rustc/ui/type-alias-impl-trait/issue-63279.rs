#![feature(type_alias_impl_trait)]

type Closure = impl FnOnce();

fn c() -> Closure {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    || -> Closure { || () }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
}

fn main() {}

