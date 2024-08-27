fn main() {
    let z = (10,);
    let _ = z[0]; // { dg-error ".E0608." "" { target *-*-* } }
}

