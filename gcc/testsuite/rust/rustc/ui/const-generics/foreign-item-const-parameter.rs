extern "C" {
    fn foo<const X: usize>(); // { dg-error ".E0044." "" { target *-*-* } }

    fn bar<T, const X: usize>(_: T); // { dg-error ".E0044." "" { target *-*-* } }
}

fn main() {}

