//@ run-rustfix

fn foo(x: &str) -> bool {
    x.starts_with("hi".to_string() + " you")
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn foo2(x: &str) -> bool {
    x.starts_with("hi".to_string())
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {
    foo("hi you");
    foo2("hi");
}

