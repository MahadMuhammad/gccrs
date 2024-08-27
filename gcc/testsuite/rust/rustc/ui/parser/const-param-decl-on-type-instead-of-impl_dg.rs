struct NInts<const N: usize>([u8; N]);
impl NInts<const N: usize> {} // { dg-error "" "" { target *-*-* } }

fn main() {
    let _: () = 42; // { dg-error ".E0308." "" { target *-*-* } }
}

fn banana(a: <T<const N: usize>>::BAR) {}
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }
fn chaenomeles() {
    path::path::Struct::<const N: usize>()
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-error ".E0433." "" { target *-*-* } .-2 }
}

