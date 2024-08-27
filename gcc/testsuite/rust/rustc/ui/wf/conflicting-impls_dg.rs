// { dg-additional-options "-frust-edition= 2021" }

struct Ty;

impl TryFrom<Ty> for u8 {
    type Error = Ty;
    fn try_from(_: Ty) -> Result<Self, Self::Error> {
        loop {}
    }
}

impl TryFrom<Ty> for u8 {
// { dg-error ".E0119." "" { target *-*-* } .-1 }
    type Error = Ty;
    fn try_from(_: Ty) -> Result<Self, Self::Error> {
        loop {}
    }
}

fn main() {}

