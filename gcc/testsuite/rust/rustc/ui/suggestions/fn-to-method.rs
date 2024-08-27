struct Foo;

impl Foo {
    fn bar(self) {}
}

fn main() {
    let x = cmp(&1, &2);
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { help ".E0425." "" { target *-*-* } .-2 }

    let y = len([1, 2, 3]);
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { help ".E0425." "" { target *-*-* } .-2 }

    let z = bar(Foo);
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { help ".E0425." "" { target *-*-* } .-2 }
}

