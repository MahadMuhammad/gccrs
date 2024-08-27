// Regression test for https://github.com/rust-lang/rust/issues/122549
// was fixed by https://github.com/rust-lang/rust/pull/122749

trait ConstChunksExactTrait<T> {
    fn const_chunks_exact<const N: usize>(&self) -> ConstChunksExact<'a, T, { N }>;
// { dg-error ".E0261." "" { target *-*-* } .-1 }
}

impl<T> ConstChunksExactTrait<T> for [T] {}
// { dg-error ".E0046." "" { target *-*-* } .-1 }
struct ConstChunksExact<'rem, T: 'a, const N: usize> {}
// { dg-error ".E0392." "" { target *-*-* } .-1 }
// { dg-error ".E0392." "" { target *-*-* } .-2 }
// { dg-error ".E0392." "" { target *-*-* } .-3 }
impl<'a, T, const N: usize> Iterator for ConstChunksExact<'a, T, {}> {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    type Item = &'a [T; N];
}

fn main() {
    let slice = &[1i32, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut iter = [[1, 2, 3], [4, 5, 6], [7, 8, 9]].iter();

    for a in slice.const_chunks_exact::<3>() {
        assert_eq!(a, iter.next().unwrap());
    }
}

