//@ compile-flags: --emit link

fn main() {
    let a: [i32; 0] = [];
    match [a[..]] {
// { dg-error ".E0508." "" { target *-*-* } .-1 }
// { dg-error ".E0508." "" { target *-*-* } .-2 }
        [[]] => (),
        _ => (),
    }
}

