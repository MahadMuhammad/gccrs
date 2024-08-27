struct S;

enum Age {
    Years(i64, i64)
}

fn foo() {
    let mut age = 29;
    Age::Years({age += 1; age}, 55)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn bar() {
    let mut age = 29;
    Age::Years(age, 55)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn baz() {
    S
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

