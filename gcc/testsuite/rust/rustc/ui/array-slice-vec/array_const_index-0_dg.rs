const A: &'static [i32] = &[];
const B: i32 = (&A)[1];
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }

fn main() {
    let _ = B;
}

