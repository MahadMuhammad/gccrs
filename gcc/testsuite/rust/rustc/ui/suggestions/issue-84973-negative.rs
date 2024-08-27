// Checks that we only suggest borrowing if &T actually implements the trait.

trait Tr {}
impl Tr for &f32 {}
fn bar<T: Tr>(t: T) {}

fn main() {
    let a = 0i32;
    let b = 0.0f32;
    bar(a); // { dg-error ".E0277." "" { target *-*-* } }
    bar(b); // { dg-error ".E0277." "" { target *-*-* } }
}

