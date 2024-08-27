use std::collections::BTreeSet;

#[derive(Hash)]
pub enum ElemDerived {
// { dg-error ".E0391." "" { target *-*-* } .-1 }
// { dg-error ".E0391." "" { target *-*-* } .-2 }
    A(ElemDerived)
}


pub enum Elem {
    Derived(ElemDerived)
}

pub struct Set(BTreeSet<Elem>);

impl Set {
    pub fn into_iter(self) -> impl Iterator<Item = Elem> {
        self.0.into_iter()
    }
}

fn main() {}

