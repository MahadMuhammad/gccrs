#[derive(Clone)]
struct P<T> {
    x: T,
    y: f64,
}

impl<T> P<T> {
    fn y(&self, y: f64) -> Self { P{y, .. self.clone() } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

