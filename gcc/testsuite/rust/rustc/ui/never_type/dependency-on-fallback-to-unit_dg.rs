//@ check-pass

fn main() {
    def();
    _ = question_mark();
}

fn def() {
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    match true {
        false => <_>::default(),
        true => return,
    };
}

// <https://github.com/rust-lang/rust/issues/51125>
// <https://github.com/rust-lang/rust/issues/39216>
fn question_mark() -> Result<(), ()> {
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    deserialize()?;
    Ok(())
}

fn deserialize<T: Default>() -> Result<T, ()> {
    Ok(T::default())
}

