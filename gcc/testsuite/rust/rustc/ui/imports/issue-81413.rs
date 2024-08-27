pub const ITEM: Item = Item;

pub struct Item;

pub fn item() {}

pub use doesnt_exist::*;
// { dg-error ".E0432." "" { target *-*-* } .-1 }
mod a {
    use crate::{item, Item, ITEM};
}

mod b {
    use crate::item;
    use crate::Item;
    use crate::ITEM;
}

mod c {
    use crate::item;
}

fn main() {}

