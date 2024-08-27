struct Uwu<const N: u32 = 1, const M: u32 = N>;

trait Trait {}
impl<const N: u32> Trait for Uwu<N> {}

fn rawr() -> impl Trait {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    Uwu::<10, 12>
}

trait Traitor<const N: u8 = 1, const M: u8 = N> {}

impl<const N: u8> Traitor<N, 2> for u32 {}
impl Traitor<1, 2> for u64 {}

fn uwu<const N: u8>() -> impl Traitor<N> {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    1_u32
}

fn owo() -> impl Traitor {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    1_u64
}

fn main() {
    rawr();
    uwu(); // { dg-error ".E0284." "" { target *-*-* } }
    owo();
}

