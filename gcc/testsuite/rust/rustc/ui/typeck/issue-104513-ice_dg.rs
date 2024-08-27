struct S;
fn f() {
    let _: S<impl Oops> = S; // { dg-error ".E0562." "" { target *-*-* } }
// { dg-error ".E0562." "" { target *-*-* } .-1 }
}
fn main() {}

