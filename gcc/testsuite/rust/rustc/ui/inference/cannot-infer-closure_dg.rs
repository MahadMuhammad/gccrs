fn main() {
    let x = |a: (), b: ()| {
        Err(a)?;
        Ok(b)
// { dg-error ".E0282." "" { target *-*-* } .-1 }
    };
}

