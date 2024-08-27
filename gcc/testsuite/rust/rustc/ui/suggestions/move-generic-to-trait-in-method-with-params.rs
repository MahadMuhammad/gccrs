// Generalizes the suggestion introduced in #100838

trait Foo<T> {
    fn bar(&self, _: T);
}

impl Foo<i32> for i32 {
    fn bar(&self, x: i32) {
        println!("{}", self + x);
    }
}

fn main() {
    1.bar::<i32>(0);
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
// { help ".E0107." "" { target *-*-* } .-3 }
}

