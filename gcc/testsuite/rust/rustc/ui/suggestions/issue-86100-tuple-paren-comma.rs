// Tests that a suggestion is issued for type mismatch errors when a
// 1-tuple is expected and a parenthesized expression of non-tuple
// type is supplied.

fn foo<T>(_t: (T,)) {}
struct S { _s: (String,) }

fn main() {
    let _x: (i32,) = (5);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }

    foo((Some(3)));
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }

    let _s = S { _s: ("abc".to_string()) };
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }

    // Do not issue the suggestion if the found type is already a tuple.
    let t = (1, 2);
    let _x: (i32,) = (t);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

