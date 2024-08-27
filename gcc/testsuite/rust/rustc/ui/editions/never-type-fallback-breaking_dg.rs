//@ revisions: e2021 e2024
//
// { dg-additional-options "-frust-edition= 2021" }
// { dg-additional-options "-frust-edition= 2024" }
//@[e2024] compile-flags: -Zunstable-options
//
//@[e2021] run-pass
//@[e2024] check-fail

fn main() {
    m();
    q();
}

fn m() {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    let x = match true {
        true => Default::default(),
// { dg-error "" "" { target *-*-* } .-1 }
        false => panic!("..."),
    };

    dbg!(x);
}

fn q() -> Option<()> {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    fn deserialize<T: Default>() -> Option<T> {
        Some(T::default())
    }

    deserialize()?;
// { dg-error "" "" { target *-*-* } .-1 }

    None
}

