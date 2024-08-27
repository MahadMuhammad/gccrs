fn foo<T: std::fmt::Display>() {}

fn main() {
    let x = foo::<()>;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

