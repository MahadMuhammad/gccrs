// Check that even though Cell: DispatchFromDyn it remains an invalid self parameter type

use std::cell::Cell;

trait Trait{
    fn cell(self: Cell<&Self>); // { dg-error ".E0307." "" { target *-*-* } }
}

fn main() {}

