fn main() {
    let mut a;
    a = a = true; // { dg-error ".E0308." "" { target *-*-* } }
}

