/* Checks all four scenarios possible in report_elision_failure() of
 * rustc_resolve::late::lifetimes::LifetimeContext related to returning
 * borrowed values, in various configurations.
 */

fn f1() -> &i32 { loop {} }
// { dg-error ".E0106." "" { target *-*-* } .-1 }
fn f1_() -> (&i32, &i32) { loop {} }
// { dg-error ".E0106." "" { target *-*-* } .-1 }

fn f2(a: i32, b: i32) -> &i32 { loop {} }
// { dg-error ".E0106." "" { target *-*-* } .-1 }
fn f2_(a: i32, b: i32) -> (&i32, &i32) { loop {} }
// { dg-error ".E0106." "" { target *-*-* } .-1 }

struct S<'a, 'b> { a: &'a i32, b: &'b i32 }
fn f3(s: &S) -> &i32 { loop {} }
// { dg-error ".E0106." "" { target *-*-* } .-1 }
fn f3_(s: &S, t: &S) -> (&i32, &i32) { loop {} }
// { dg-error ".E0106." "" { target *-*-* } .-1 }

fn f4<'a, 'b>(a: &'a i32, b: &'b i32) -> &i32 { loop {} }
// { dg-error ".E0106." "" { target *-*-* } .-1 }
fn f4_<'a, 'b>(a: &'a i32, b: &'b i32) -> (&i32, &i32) { loop {} }
// { dg-error ".E0106." "" { target *-*-* } .-1 }

fn f5<'a>(a: &'a i32, b: &i32) -> &i32 { loop {} }
// { dg-error ".E0106." "" { target *-*-* } .-1 }
fn f5_<'a>(a: &'a i32, b: &i32) -> (&i32, &i32) { loop {} }
// { dg-error ".E0106." "" { target *-*-* } .-1 }

fn main() {}

