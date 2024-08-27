#![deny(uncommon_codepoints)]

const µ: f64 = 0.000001; // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }

fn dĳkstra() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {
    let ㇻㇲㇳ = "rust"; // { dg-error "" "" { target *-*-* } }

    // using the same identifier the second time won't trigger the lint.
    println!("{}", ㇻㇲㇳ);
}

