// Regression test for #87549.
//@ incremental

pub struct Table<T, const N: usize>([Option<T>; N]);

impl<'a, T, const N: usize> IntoIterator for &'a Table<T, N> {
    type IntoIter = std::iter::Flatten<std::slice::Iter<'a, T>>; // { dg-error ".E0277." "" { target *-*-* } }
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter { // { dg-error ".E0277." "" { target *-*-* } }
        unimplemented!()
    }
}
fn main() {}

