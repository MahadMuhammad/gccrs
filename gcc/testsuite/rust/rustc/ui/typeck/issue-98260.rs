fn main() {}
trait A {
    fn a(aa: B) -> Result<_, B> {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
        Ok(())
    }
}

enum B {}

