#[derive(Clone, Copy)]
// { dg-error ".E0204." "" { target *-*-* } .-1 }
struct Foo(N, NotDefined, <i32 as Iterator>::Item, Vec<i32>, String);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
// { dg-error ".E0277." "" { target *-*-* } .-4 }
// { dg-error ".E0277." "" { target *-*-* } .-5 }
// { dg-error ".E0277." "" { target *-*-* } .-6 }

#[derive(Clone, Copy)]
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
struct Bar<T>(T, N, NotDefined, <i32 as Iterator>::Item, Vec<i32>, String);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
// { dg-error ".E0277." "" { target *-*-* } .-4 }

fn main() {}

