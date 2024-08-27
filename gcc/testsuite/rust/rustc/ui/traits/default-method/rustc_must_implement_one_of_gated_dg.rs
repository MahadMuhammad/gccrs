#[rustc_must_implement_one_of(eq, neq)]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
trait Equal {
    fn eq(&self, other: &Self) -> bool {
        !self.neq(other)
    }

    fn neq(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

fn main() {}

