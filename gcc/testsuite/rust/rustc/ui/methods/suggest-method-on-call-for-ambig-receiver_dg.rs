// Fix for <https://github.com/rust-lang/rust/issues/125432>.

fn separate_arms() {
    let mut x = None;
    match x {
        None => {
            x = Some(0);
        }
        Some(right) => {
            consume(right);
// { dg-error ".E0425." "" { target *-*-* } .-1 }
        }
    }
}

fn main() {}

