struct S(i32, f32);
enum E {
    S(i32, f32),
}
fn main() {
    let x = E::S(1, 2.2);
    match x {
        E::S { 0, 1 } => {}
// { dg-error ".E0769." "" { target *-*-* } .-1 }
    }
    let y = S(1, 2.2);
    match y {
        S { } => {} // { dg-error ".E0769." "" { target *-*-* } }
    }

    if let E::S { 0: a } = x { // { dg-error ".E0027." "" { target *-*-* } }
    }
}

