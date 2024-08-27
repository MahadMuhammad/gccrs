fn foo<U>() {}

fn main() {
    foo::<main>()
// { dg-error ".E0747." "" { target *-*-* } .-1 }
}

