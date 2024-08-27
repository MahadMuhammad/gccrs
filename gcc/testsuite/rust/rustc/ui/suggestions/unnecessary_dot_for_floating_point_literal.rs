fn main() {
    let _: f64 = 0..10; // { dg-error ".E0308." "" { target *-*-* } }
    let _: f64 = 1..; // { dg-error ".E0308." "" { target *-*-* } }
    let _: f64 = ..10; // { dg-error ".E0308." "" { target *-*-* } }
    let _: f64 = std::ops::Range { start: 0, end: 1 }; // { dg-error ".E0308." "" { target *-*-* } }
}

