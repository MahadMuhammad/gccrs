impl X {
// { dg-error ".E0412." "" { target *-*-* } .-1 }
    fn getn<const N: usize>() -> [u8; N] {
        getn::<N>()
    }
}
fn getn<const N: cfg_attr>() -> [u8; N] {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }

fn main() {}

