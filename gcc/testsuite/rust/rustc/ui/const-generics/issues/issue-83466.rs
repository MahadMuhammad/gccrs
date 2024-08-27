// regression test for #83466- tests that generic arg mismatch errors between
// consts and types are not suppressed when there are explicit late bound lifetimes

struct S;
impl S {
    fn func<'a, U>(self) -> U {
        todo!()
    }
}
fn dont_crash<'a, U>() {
    S.func::<'a, 10_u32>()
// { dg-warning ".E0747." "" { target *-*-* } .-1 }
// { dg-warning ".E0747." "" { target *-*-* } .-2 }
// { dg-error ".E0747." "" { target *-*-* } .-3 }
}

fn main() {}

