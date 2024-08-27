trait Trait {
    type Item;
}

trait Trait2 {}

// It's not possible to insert a universal `impl Trait` here!
impl dyn Trait<Item: Trait2> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

