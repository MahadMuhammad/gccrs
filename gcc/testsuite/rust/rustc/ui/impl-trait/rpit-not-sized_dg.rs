fn foo() -> impl ?Sized {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    ()
}

fn main() {}

