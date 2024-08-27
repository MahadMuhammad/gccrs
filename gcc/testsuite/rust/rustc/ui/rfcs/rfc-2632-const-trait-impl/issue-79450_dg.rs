//@ compile-flags: -Znext-solver
#![allow(incomplete_features)]
#![feature(const_fmt_arguments_new)]
#![feature(const_trait_impl, effects)]

#[const_trait]
trait Tr {
    fn req(&self);

    fn prov(&self) {
        println!("lul"); // { dg-error ".E0015." "" { target *-*-* } }
        self.req();
    }
}

struct S;

impl const Tr for S {
    fn req(&self) {}
}

fn main() {}

