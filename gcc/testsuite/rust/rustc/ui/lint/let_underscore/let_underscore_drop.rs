//@ check-pass
#![warn(let_underscore_drop)]

struct NontrivialDrop;

impl Drop for NontrivialDrop {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() {
    let _ = NontrivialDrop; // { dg-warning "" "" { target *-*-* } }

    let (_, _) = (NontrivialDrop, NontrivialDrop); // This should be ignored.
}

