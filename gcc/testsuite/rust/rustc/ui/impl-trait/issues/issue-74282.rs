#![feature(type_alias_impl_trait)]

type Closure = impl Fn() -> u64;
struct Anonymous(Closure);

fn bop(_: Closure) {
    let y = || -> Closure { || 3 };
    Anonymous(|| {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    })
}

fn main() {}

