fn main() {
    if true {
    } else if {
// { dg-error "" "" { target *-*-* } .-1 }
    } else {
    }
}

fn foo() {
    if true {
    } else if {
// { dg-error "" "" { target *-*-* } .-1 }
    }
    bar();
}

fn bar() {}

