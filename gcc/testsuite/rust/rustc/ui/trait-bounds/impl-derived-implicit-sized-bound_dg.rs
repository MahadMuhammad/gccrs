struct Victim<'a, T: Perpetrator + ?Sized>
where
  Self: Sized
{
  value: u8,
  perp: &'a T,
}

trait VictimTrait {
  type Ret;
  fn get(self) -> Self::Ret;
}

// Actual fix is here
impl<'a, T: Perpetrator /*+ ?Sized*/> VictimTrait for Victim<'a, T> {
  type Ret = u8;
  fn get(self) -> Self::Ret {
    self.value
  }
}

trait Perpetrator {
  fn getter<'a>(&'a self) -> Victim<'a, Self> {
    Victim {
      value: 0,
      perp: self,
    }
  }

  fn trigger(&self) {
    self.getter().get();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
  }
}

fn main() {}

