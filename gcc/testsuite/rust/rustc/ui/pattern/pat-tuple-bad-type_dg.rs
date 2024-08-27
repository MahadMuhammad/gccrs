fn main() {
    let x; // { dg-error ".E0282." "" { target *-*-* } }

    match x {
        (..) => {}
        _ => {}
    }

    match 0u8 {
        (..) => {} // { dg-error ".E0308." "" { target *-*-* } }
        _ => {}
    }

    x = 10;
}

