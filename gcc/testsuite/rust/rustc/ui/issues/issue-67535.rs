fn main() {}

impl std::ops::AddAssign for () {
// { dg-error ".E0117." "" { target *-*-* } .-1 }
    fn add_assign(&self, other: ()) -> () {
        ()
    }
}

impl std::ops::AddAssign for [(); 1] {
// { dg-error ".E0117." "" { target *-*-* } .-1 }
    fn add_assign(&self, other: [(); 1]) -> [(); 1] {
        [()]
    }
}

impl std::ops::AddAssign for &[u8] {
// { dg-error ".E0117." "" { target *-*-* } .-1 }
    fn add_assign(&self, other: &[u8]) -> &[u8] {
        self
    }
}

