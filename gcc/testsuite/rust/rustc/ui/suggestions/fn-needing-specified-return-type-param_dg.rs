fn f<A>() -> A { unimplemented!() }
fn foo() {
    let _ = f;
// { dg-error ".E0282." "" { target *-*-* } .-1 }
// { help ".E0282." "" { target *-*-* } .-2 }
}
fn main() {}

