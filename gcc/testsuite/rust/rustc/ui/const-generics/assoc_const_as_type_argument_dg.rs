trait Trait {
    const ASSOC: usize;
}

fn bar<const N: usize>() {}

fn foo<T: Trait>() {
    bar::<<T as Trait>::ASSOC>();
// { dg-error ".E0747." "" { target *-*-* } .-1 }
// { dg-error ".E0747." "" { target *-*-* } .-2 }
}

fn main() {}

