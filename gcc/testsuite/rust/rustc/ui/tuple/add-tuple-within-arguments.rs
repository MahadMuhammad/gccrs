fn foo(s: &str, a: (i32, i32), s2: &str) {}

fn bar(s: &str, a: (&str,), s2: &str) {}

fn main() {
    foo("hi", 1, 2, "hi");
// { dg-error ".E0061." "" { target *-*-* } .-1 }
    bar("hi", "hi", "hi");
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

