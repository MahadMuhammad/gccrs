// Tests that the compiler suggests an `into_iter` call when an `Iterator` method
// is called on something that implements `IntoIterator`

fn main() {
    let items = items();
    let other_items = items.map(|i| i + 1);
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    let vec: Vec<i32> = items.collect();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

fn items() -> impl IntoIterator<Item = i32> {
    vec![1, 2, 3]
}

fn process(items: impl IntoIterator<Item = String>) -> Vec<String> {
    items.collect()
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

