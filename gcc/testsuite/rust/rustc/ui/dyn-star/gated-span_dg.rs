macro_rules! t {
    ($t:ty) => {}
}

t!(dyn* Send);
// { dg-error ".E0658." "" { target *-*-* } .-1 }

fn main() {}

