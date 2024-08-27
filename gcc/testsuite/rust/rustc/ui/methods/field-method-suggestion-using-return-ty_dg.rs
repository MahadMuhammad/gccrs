struct Wrapper<T>(T);

impl Wrapper<Option<i32>> {
    fn inner_mut(&self) -> Option<&mut i32> {
        self.as_mut()
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
// { help ".E0599." "" { target *-*-* } .-3 }
    }

    fn inner_mut_bad(&self) -> Option<&mut u32> {
        self.as_mut()
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
    }
}

fn main() {}

