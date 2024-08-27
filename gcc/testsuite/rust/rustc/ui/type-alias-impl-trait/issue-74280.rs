// Regression test for #74280.

#![feature(type_alias_impl_trait)]

type Test = impl Copy;

fn test() -> Test {
    let y = || -> Test { () };
    7 // { dg-error ".E0308." "" { target *-*-* } }
}

fn main() {}

