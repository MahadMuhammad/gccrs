trait Deserialize {
    fn deserialize(&self);
}

struct ArchivedVec<T>(T);

impl<T> Deserialize for ArchivedVec<T> {
    fn deserialize(s: _) {}
// { dg-error ".E0186." "" { target *-*-* } .-1 }
// { dg-error ".E0186." "" { target *-*-* } .-2 }
}

fn main() {}

