trait TraitA<T> {
    fn func();
}

struct StructA {}

impl TraitA<i32> for StructA {
    fn func() {}
}

fn main() {
    TraitA::<i32>::func();
// { dg-error ".E0790." "" { target *-*-* } .-1 }
// { help ".E0790." "" { target *-*-* } .-2 }
}

