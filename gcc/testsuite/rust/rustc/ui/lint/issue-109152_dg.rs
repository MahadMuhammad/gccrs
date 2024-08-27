#![deny(map_unit_fn)]

#![crate_type = "lib"]
fn _y() {
    vec![42].iter().map(drop);
// { dg-error "" "" { target *-*-* } .-1 }
}

