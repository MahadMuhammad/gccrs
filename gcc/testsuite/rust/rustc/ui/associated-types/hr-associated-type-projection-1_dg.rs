trait UnsafeCopy<'a, T: Copy>
where
    for<'b> <Self as UnsafeCopy<'b, T>>::Item: std::ops::Deref<Target = T>,
{
    type Item;

    fn bug(item: &Self::Item) -> () {
        let x: T = **item;
        &x as *const _;
    }
}

impl<T: Copy + std::ops::Deref> UnsafeCopy<'_, T> for T {
    type Item = T;
// { dg-error ".E0271." "" { target *-*-* } .-1 }
}

pub fn main() {
    <&'static str>::bug(&"");
// { dg-error ".E0271." "" { target *-*-* } .-1 }
}

