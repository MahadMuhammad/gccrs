fn main() {
    let _ = {42}(); // { dg-error ".E0618." "" { target *-*-* } }
}

