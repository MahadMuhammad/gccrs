#![feature(const_trait_impl)]
#![feature(effects)] // { dg-warning "" "" { target *-*-* } }

struct S;
trait T {}

impl const dyn T {
// { dg-error "" "" { target *-*-* } .-1 }
    pub const fn new() -> std::sync::Mutex<dyn T> {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
}

fn main() {}

