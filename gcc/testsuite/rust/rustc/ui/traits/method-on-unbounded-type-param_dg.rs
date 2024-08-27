fn f<T>(a: T, b: T) -> std::cmp::Ordering {
    a.cmp(&b) // { dg-error ".E0599." "" { target *-*-* } }
}
fn g<T>(a: T, b: T) -> std::cmp::Ordering {
    (&a).cmp(&b) // { dg-error ".E0599." "" { target *-*-* } }
}
fn h<T>(a: &T, b: T) -> std::cmp::Ordering {
    a.cmp(&b) // { dg-error ".E0599." "" { target *-*-* } }
}
trait T {}
impl<X: std::cmp::Ord> T for X {}
fn main() {
    let x: Box<dyn T> = Box::new(0);
    x.cmp(&x); // { dg-error ".E0599." "" { target *-*-* } }
}

