pub trait Insertable {
    type Values;

    fn values(&self) -> Self::Values;
}

impl<T> Insertable for Option<T> {
    type Values = ();

    fn values(self) -> Self::Values {
// { dg-error ".E0053." "" { target *-*-* } .-1 }
        self.map(Insertable::values).unwrap_or_default()
// { dg-error ".E0631." "" { target *-*-* } .-1 }
    }
}

fn main() {}

