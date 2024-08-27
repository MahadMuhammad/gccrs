//@ error-pattern: r#T: 'static
//@ error-pattern: r#K: 'static
//@ error-pattern: T: 'static=

// https://github.com/rust-lang/rust/issues/124785

struct Foo<T, K = i32>(&'static T, &'static K);
// { dg-error ".E0310." "" { target *-*-* } .-1 }
// { dg-error ".E0310." "" { target *-*-* } .-2 }

struct Bar<r#T, r#K = i32>(&'static r#T, &'static r#K);
// { dg-error ".E0310." "" { target *-*-* } .-1 }
// { dg-error ".E0310." "" { target *-*-* } .-2 }

struct Boo<T= i32>(&'static T);
// { dg-error ".E0310." "" { target *-*-* } .-1 }

struct Far<T
= i32>(&'static T);
// { dg-error ".E0310." "" { target *-*-* } .-1 }

struct S<'a, K: 'a = i32>(&'static K);
// { dg-error ".E0392." "" { target *-*-* } .-1 }
// { dg-error ".E0392." "" { target *-*-* } .-2 }

fn main() {}

