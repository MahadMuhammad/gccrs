#![allow(incomplete_features)]
#![feature(adt_const_params, unsized_const_params)]

union Union {
    a: u8,
}

impl PartialEq for Union {
    fn eq(&self, other: &Union) -> bool {
        true
    }
}
impl Eq for Union {}

impl std::marker::UnsizedConstParamTy for Union {}
// { dg-error "" "" { target *-*-* } .-1 }

#[derive(std::marker::UnsizedConstParamTy)]
// { dg-error "" "" { target *-*-* } .-1 }
union UnionDerive {
    a: u8,
}

impl PartialEq for UnionDerive {
    fn eq(&self, other: &UnionDerive) -> bool {
        true
    }
}
impl Eq for UnionDerive {}

fn main() {}

