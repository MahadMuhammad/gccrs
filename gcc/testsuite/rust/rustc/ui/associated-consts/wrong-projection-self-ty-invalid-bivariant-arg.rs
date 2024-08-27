struct Fail<T>;
// { dg-error ".E0392." "" { target *-*-* } .-1 }

impl Fail<i32> {
    const C: () = ();
}

fn main() {
    Fail::<()>::C
}

