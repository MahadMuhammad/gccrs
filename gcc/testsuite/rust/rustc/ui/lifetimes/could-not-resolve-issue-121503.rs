// { dg-additional-options "-frust-edition=2018" }

#![feature(allocator_api)]
struct Struct;
impl Struct {
    async fn box_ref_Struct(self: Box<Self, impl FnMut(&mut Self)>) -> &u32 {
// { dg-error ".E0658." "" { target *-*-* } .-1 }
        &1
    }
}

fn main() {}

