type Array<T, const N: usize> = [T; N];

fn foo<const N: usize>() -> Array<N, ()> {
// { dg-error ".E0747." "" { target *-*-* } .-1 }
    unimplemented!()
}

fn main() {}

