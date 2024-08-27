// Regression test for #87017.

//@ run-rustfix

fn main() {
    fn foo() -> Vec<i32> { vec![1, 2, 3] }

    if let [_, _, _] = foo() {}
// { dg-error ".E0529." "" { target *-*-* } .-1 }
// { help ".E0529." "" { target *-*-* } .-2 }

    if let [] = &foo() {}
// { dg-error ".E0529." "" { target *-*-* } .-1 }
// { help ".E0529." "" { target *-*-* } .-2 }

    if let [] = foo() {}
// { dg-error ".E0529." "" { target *-*-* } .-1 }
// { help ".E0529." "" { target *-*-* } .-2 }

    let v = vec![];
    match &v {
// { help "" "" { target *-*-* } .-1 }
        [5] => {}
// { dg-error ".E0529." "" { target *-*-* } .-1 }
        _ => {}
    }

    let [..] = vec![1, 2, 3];
// { dg-error ".E0529." "" { target *-*-* } .-1 }
// { help ".E0529." "" { target *-*-* } .-2 }
}

