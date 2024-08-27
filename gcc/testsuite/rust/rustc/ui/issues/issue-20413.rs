//@ normalize-stderr-test: "long-type-\d+" -> "long-type-hash"
trait Foo {
    fn answer(self);
}

struct NoData<T>;
// { dg-error ".E0392." "" { target *-*-* } .-1 }

impl<T> Foo for T where NoData<T>: Foo {
// { dg-error ".E0275." "" { target *-*-* } .-1 }
  fn answer(self) {
    let val: NoData<T> = NoData;
  }
}

trait Bar {
    fn answer(self);
}

trait Baz {
    fn answer(self);
}

struct AlmostNoData<T>(Option<T>);

struct EvenLessData<T>(Option<T>);

impl<T> Bar for T where EvenLessData<T>: Baz {
// { dg-error ".E0275." "" { target *-*-* } .-1 }
  fn answer(self) {
    let val: EvenLessData<T> = EvenLessData(None);
  }
}

impl<T> Baz for T where AlmostNoData<T>: Bar {
// { dg-error ".E0275." "" { target *-*-* } .-1 }
  fn answer(self) {
    let val: NoData<T> = AlmostNoData(None);
  }
}

fn main() {}

