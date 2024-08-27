fn foo(x: &Vec<i32>) -> impl Sized {
    x
// { dg-error ".E0700." "" { target *-*-* } .-1 }
}

fn main() {}

