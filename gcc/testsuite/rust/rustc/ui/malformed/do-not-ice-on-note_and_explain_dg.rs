struct A<B>(B);

impl<B> A<B> {
    fn d() {
        fn d() {
            Self(1)
// { dg-error ".E0401." "" { target *-*-* } .-1 }
        }
    }
}

fn main() {}

