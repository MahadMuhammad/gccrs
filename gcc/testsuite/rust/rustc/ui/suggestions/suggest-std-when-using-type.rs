//@ run-rustfix
fn main() {
    let pi = f32::consts::PI; // { dg-error ".E0223." "" { target *-*-* } }
    let bytes = "hello world".as_bytes();
    let string = str::from_utf8(bytes).unwrap();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    println!("{pi} {bytes:?} {string}");
}

