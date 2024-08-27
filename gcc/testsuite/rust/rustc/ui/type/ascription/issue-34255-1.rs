// rustfix

struct Reactor {
    input_cells: Vec<usize>,
}

impl Reactor {
    pub fn new() -> Self { // { dg-error "" "" { target *-*-* } }
        input_cells: Vec::new()
    }
}

// This case isn't currently being handled gracefully, including for completeness.
fn main() {}

