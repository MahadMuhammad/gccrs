extern "C" {
    fn foo<T>(); // { dg-error ".E0044." "" { target *-*-* } }
}

fn main() {
    foo::<i32>(); // { dg-error ".E0133." "" { target *-*-* } }
}

