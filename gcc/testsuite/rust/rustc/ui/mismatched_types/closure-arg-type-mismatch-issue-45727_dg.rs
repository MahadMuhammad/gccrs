//@ run-rustfix
fn main() {
    let _ = (-10..=10).find(|x: i32| x.signum() == 0); // { dg-error ".E0631." "" { target *-*-* } }
    let _ = (-10..=10).find(|x: &&&i32| x.signum() == 0); // { dg-error ".E0631." "" { target *-*-* } }
}

