// https://doc.rust-lang.org/error_codes/E0061.html
fn main() {
fn f(u: i32) {}

f(); // { dg-error "unexpected number of arguments 0 expected 1" }
}