pub struct Example<const N: usize = 13>;
pub struct Example2<T = u32, const N: usize = 13>(T);
pub struct Example3<const N: usize = 13, T = u32>(T);
pub struct Example4<const N: usize = 13, const M: usize = 4>;

fn main() {
    let e: Example<13> = ();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    let e: Example2<u32, 13> = ();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    let e: Example3<13, u32> = ();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    let e: Example3<7> = ();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    let e: Example4<7> = ();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
}

