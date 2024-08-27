//@ forbid-output: &mut mut self

struct Struct;

impl Struct {
    fn foo(&mut self) {
        (&mut self).bar(); // { dg-error ".E0596." "" { target *-*-* } }
// { help ".E0596." "" { target *-*-* } .-1 }
    }

    // In this case we could keep the suggestion, but to distinguish the
    // two cases is pretty hard. It's an obscure case anyway.
    fn bar(self: &mut Self) {
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        (&mut self).bar(); // { dg-error ".E0596." "" { target *-*-* } }
// { help ".E0596." "" { target *-*-* } .-1 }
    }
}

fn main () {}

