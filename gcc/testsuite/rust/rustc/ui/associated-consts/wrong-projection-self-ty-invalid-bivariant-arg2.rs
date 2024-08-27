trait Proj {
    type Assoc;
}
impl<T> Proj for T {
    type Assoc = T;
}

struct Fail<T: Proj<Assoc = U>, U>(T);

impl Fail<i32, i32> {
    const C: () = ();
}

fn main() {
    Fail::<i32, u32>::C
// { dg-error ".E0271." "" { target *-*-* } .-1 }
}

