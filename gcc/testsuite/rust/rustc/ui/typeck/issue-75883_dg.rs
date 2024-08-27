// Regression test for #75883.

pub struct UI {}

impl UI {
    pub fn run() -> Result<_> {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-error ".E0121." "" { target *-*-* } .-2 }
        let mut ui = UI {};
        ui.interact();

        unimplemented!();
    }

    pub fn interact(&mut self) -> Result<_> {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-error ".E0121." "" { target *-*-* } .-2 }
        unimplemented!();
    }
}

fn main() {}

