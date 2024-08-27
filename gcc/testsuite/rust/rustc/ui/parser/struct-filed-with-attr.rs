// Issue: 100461, Try to give a helpful diagnostic even when the next struct field has an attribute.
//@ run-rustfix

struct Feelings {
    owo: bool
// { dg-error "" "" { target *-*-* } .-1 }
    #[allow(unused)]
    uwu: bool,
}

impl Feelings {
    #[allow(unused)]
    fn hmm(&self) -> bool {
        self.owo
    }
}

fn main() { }

