fn foo<const N: usize>() -> [u8; N] {
    bar::<N>()
// { dg-error "" "" { target *-*-* } .-1 }
}

fn bar<const N: u8>() -> [u8; N] {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }

fn main() {}

