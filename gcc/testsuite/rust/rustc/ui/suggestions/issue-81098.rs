// Don't suggest removing a semicolon if the last statement isn't an expression with semicolon
// (#81098)
fn wat() -> impl core::fmt::Display { // { dg-error ".E0277." "" { target *-*-* } }
    fn why() {}
}

// Do it if the last statement is an expression with semicolon
// (#54771)
fn ok() -> impl core::fmt::Display { // { dg-error ".E0277." "" { target *-*-* } }
    1;
}

fn main() {}

