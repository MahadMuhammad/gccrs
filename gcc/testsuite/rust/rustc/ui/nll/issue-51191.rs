struct Struct;

impl Struct {
    fn bar(self: &mut Self) {
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        (&mut self).bar();
// { dg-error ".E0596." "" { target *-*-* } .-1 }
// { help ".E0596." "" { target *-*-* } .-2 }
    }

    fn imm(self) { // { help "" "" { target *-*-* } }
        (&mut self).bar();
// { dg-error ".E0596." "" { target *-*-* } .-1 }
    }

    fn mtbl(mut self) {
        (&mut self).bar();
    }

    fn immref(&self) {
        (&mut self).bar();
// { dg-error ".E0596." "" { target *-*-* } .-1 }
// { dg-error ".E0596." "" { target *-*-* } .-2 }
    }

    fn mtblref(&mut self) {
        (&mut self).bar();
// { dg-error ".E0596." "" { target *-*-* } .-1 }
// { help ".E0596." "" { target *-*-* } .-2 }
    }
}

fn main() {}

