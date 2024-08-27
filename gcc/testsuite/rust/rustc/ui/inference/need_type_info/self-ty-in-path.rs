// Test that we don't ICE when encountering a `Self` in a path.
struct TestErr<T>(T);

impl<T> TestErr<T> {
    fn func_a<U>() {}

    fn func_b() {
        Self::func_a();
// { dg-error ".E0282." "" { target *-*-* } .-1 }
    }
}

fn main() {}

