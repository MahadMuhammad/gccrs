use std::{fs, io::*};
use std::collections::HashMap;

type Handle = BufWriter<fs::File>;
struct Thing(HashMap<String, Handle>);

impl Thing {
    pub fn die_horribly(&mut self) {
        for v in self.0.values() {
            v.flush();
// { dg-error ".E0596." "" { target *-*-* } .-1 }
        }
    }
}

fn main() {}

