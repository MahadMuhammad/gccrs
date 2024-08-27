// { dg-additional-options "-frust-edition=2021" }

fn main() {
    let n = 2;
    match n {
        0...3 => {}
// { dg-error ".E0783." "" { target *-*-* } .-1 }
        4...10 => {}
// { dg-error ".E0783." "" { target *-*-* } .-1 }
        (11...100) => {}
// { dg-error ".E0783." "" { target *-*-* } .-1 }
        _ => {}
    }
}

