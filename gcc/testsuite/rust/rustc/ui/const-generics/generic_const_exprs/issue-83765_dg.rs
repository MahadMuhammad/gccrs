#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

trait TensorDimension {
    const DIM: usize;
}

trait TensorSize: TensorDimension {
    fn size(&self) -> [usize; Self::DIM];
}

trait Broadcastable: TensorSize + Sized {
    type Element;
    fn lazy_updim<const NEWDIM: usize>(&self, size: [usize; NEWDIM]) {}
}

struct BMap<'a, R, T: Broadcastable, F: Fn(T::Element) -> R, const DIM: usize> {
    reference: &'a T,
    closure: F,
}

impl<'a, R, T: Broadcastable, F: Fn(T::Element) -> R, const DIM: usize> TensorDimension
    for BMap<'a, R, T, F, DIM>
{
    const DIM: usize = DIM;
}
impl<'a, R, T: Broadcastable, F: Fn(T::Element) -> R, const DIM: usize> TensorSize
    for BMap<'a, R, T, F, DIM>
{
    fn size(&self) -> [usize; DIM] {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        self.reference.size()
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    }
}

fn main() {}

