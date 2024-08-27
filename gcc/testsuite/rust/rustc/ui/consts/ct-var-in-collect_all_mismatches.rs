struct Foo<T, const N: usize> {
    array: [T; N],
}

trait Bar<const N: usize> {}

impl<T, const N: usize> Foo<T, N> {
    fn trigger(self) {
        self.unsatisfied()
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    }

    fn unsatisfied(self)
    where
        T: Bar<N>,
    {
    }
}

fn main() {}

