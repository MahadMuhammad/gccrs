// { dg-additional-options "-frust-edition= 2021" }
//@revisions: infer no_infer

#![feature(type_alias_impl_trait)]
type Bar = impl std::fmt::Display;
// { dg-error "" "" { target *-*-* } .-1 }

async fn test<const N: crate::Bar>() {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-2 }
    #[cfg(infer)]
    let x: u32 = N;
}

fn main() {}

