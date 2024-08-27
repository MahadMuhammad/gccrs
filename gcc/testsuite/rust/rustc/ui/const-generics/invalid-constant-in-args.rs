use std::cell::Cell;

fn main() {
    let _: Cell<&str, "a"> = Cell::new("");
// { dg-error ".E0107." "" { target *-*-* } .-1 }
}

