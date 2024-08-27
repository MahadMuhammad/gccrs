fn main() {
    let x = Some(123);

    if let Some(_) == x {}
// { dg-error "" "" { target *-*-* } .-1 }

    if Some(_) = x {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }

    if None = x { }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

