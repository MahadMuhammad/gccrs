fn main() {
}

fn foo<T>(x: T, y: T) {
    let z = x + y; // { dg-error ".E0369." "" { target *-*-* } }
}

fn bar<T>(x: T) {
    x += x; // { dg-error ".E0368." "" { target *-*-* } }
}

fn baz<T>(x: T) {
    let y = -x; // { dg-error ".E0600." "" { target *-*-* } }
    let y = !x; // { dg-error ".E0600." "" { target *-*-* } }
    let y = *x; // { dg-error ".E0614." "" { target *-*-* } }
}

