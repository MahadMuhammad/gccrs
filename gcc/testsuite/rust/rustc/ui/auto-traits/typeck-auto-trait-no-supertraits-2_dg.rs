#![feature(auto_traits)]
#![feature(negative_impls)]

auto trait Magic : Sized where Option<Self> : Magic {} // { dg-error ".E0568." "" { target *-*-* } }
// { dg-error ".E0568." "" { target *-*-* } .-1 }
impl<T:Magic> Magic for T {}

fn copy<T: Magic>(x: T) -> (T, T) { (x, x) }
// { dg-error ".E0382." "" { target *-*-* } .-1 }

#[derive(Debug)]
struct NoClone;

fn main() {
    let (a, b) = copy(NoClone);
    println!("{:?} {:?}", a, b);
}

