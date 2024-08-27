//@ run-rustfix

fn main() {
    match Some(1) { // { dg-error ".E0004." "" { target *-*-* } }
        Some(1) => {}
        // hello
        Some(_) => {}
    }
}

