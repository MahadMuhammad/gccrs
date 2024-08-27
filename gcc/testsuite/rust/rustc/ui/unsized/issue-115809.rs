//@ compile-flags: --emit=link -Zmir-opt-level=2 -Zpolymorphize=on

fn foo<T>() {
    let a: [i32; 0] = [];
    match [a[..]] {
// { dg-error ".E0508." "" { target *-*-* } .-1 }
// { dg-error ".E0508." "" { target *-*-* } .-2 }
        [[x]] => {}
        _ => (),
    }
}

fn main() {}

