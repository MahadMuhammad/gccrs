#![allow(unused)]

pub trait Trait  {
    type Assoc<'a> where Self: 'a;

    fn f(&self) -> Self::Assoc<'_>;

    // Disallow elision in return position, for now
    fn g(&self) -> Self::Assoc;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
}

pub struct Struct {
    item: f32
}

pub struct GenericStruct<'a> {
    ref_item: &'a f32
}

impl Trait for Struct {
    type Assoc<'a> = GenericStruct<'a>;

    fn f(&self) -> Self::Assoc<'_> {
        Self::Assoc {
            ref_item: &self.item
        }
    }

    // Disallow elision in return position, for now
    fn g(&self) -> Self::Assoc {
// { dg-error ".E0107." "" { target *-*-* } .-1 }
        todo!()
    }
}

fn main() {}

