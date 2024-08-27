type Arr<const N: usize> = [u8; N - 1];
// { dg-error "" "" { target *-*-* } .-1 }

fn test<const N: usize>() -> Arr<N> where Arr<N>: Default {
    Default::default()
}

fn main() {
    let x = test::<33>();
    assert_eq!(x, [0; 32]);
}

