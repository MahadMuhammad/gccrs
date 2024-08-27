//@ build-fail

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let _ = &xs;
    let _ = xs[7]; // { dg-error "" "" { target *-*-* } }
}

