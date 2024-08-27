fn main() {
    let _ = match Some(42) { Some(x) => x, None => "" }; // { dg-error ".E0308." "" { target *-*-* } }
}

