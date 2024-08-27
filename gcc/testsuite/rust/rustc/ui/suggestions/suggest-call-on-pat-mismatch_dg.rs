enum E {
    One(i32, i32),
}

fn main() {
    let var = E::One;
    if let E::One(var1, var2) = var {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
        println!("{var1} {var2}");
    }

    let Some(x) = Some;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
}

