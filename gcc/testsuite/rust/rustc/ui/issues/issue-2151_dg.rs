fn main() {
    let x = panic!(); // { dg-error ".E0282." "" { target *-*-* } }
    x.clone();
}

