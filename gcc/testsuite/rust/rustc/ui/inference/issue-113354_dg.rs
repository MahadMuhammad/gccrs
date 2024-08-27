//@run-rustfix
fn main() {
    let _ = || { while Some(_) = Some(1) { } }; // { dg-error ".E0308." "" { target *-*-* } }
}

