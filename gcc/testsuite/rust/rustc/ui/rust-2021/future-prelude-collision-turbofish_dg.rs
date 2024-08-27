// See https://github.com/rust-lang/rust/issues/88442
//@ run-rustfix
// { dg-additional-options "-frust-edition=2018" }
//@ check-pass
#![allow(unused)]
#![warn(rust_2021_prelude_collisions)]

trait AnnotatableTryInto {
    fn try_into<T>(self) -> Result<T, Self::Error>
    where Self: std::convert::TryInto<T> {
        std::convert::TryInto::try_into(self)
    }
}

impl<T> AnnotatableTryInto for T where T: From<u8> {}

fn main() -> Result<(), &'static str> {
    let x: u64 = 1;
    x.try_into::<usize>().or(Err("foo"))?.checked_sub(1);
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    x.try_into::<usize>().or(Err("foo"))?;
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    Ok(())
}

