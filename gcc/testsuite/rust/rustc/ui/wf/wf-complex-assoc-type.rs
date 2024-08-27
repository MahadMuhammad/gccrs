trait MyTrait {}
struct AssertMyTrait<T: MyTrait>(T);

trait HelperTrait {
    type MyItem;
}

impl HelperTrait for () {
    type MyItem = Option<((AssertMyTrait<bool>, u8))>; // { dg-error ".E0277." "" { target *-*-* } }
}

fn main() {}

