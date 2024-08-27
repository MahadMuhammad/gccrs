//@ build-fail

fn foo<T>() {
    const { assert!(std::mem::size_of::<T>() == 0); } // { dg-error ".E0080." "" { target *-*-* } }
}

fn bar<const N: usize>() -> usize {
    const { N - 1 } // { dg-error ".E0080." "" { target *-*-* } }
}

fn main() {
    foo::<i32>();
    bar::<0>();
}

