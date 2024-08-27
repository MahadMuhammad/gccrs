//@ check-pass
#![feature(do_not_recommend)]

#[diagnostic::do_not_recommend]
// { dg-warning "" "" { target *-*-* } .-1 }
const CONST: () = ();

#[diagnostic::do_not_recommend]
// { dg-warning "" "" { target *-*-* } .-1 }
static STATIC: () = ();

#[diagnostic::do_not_recommend]
// { dg-warning "" "" { target *-*-* } .-1 }
type Type = ();

#[diagnostic::do_not_recommend]
// { dg-warning "" "" { target *-*-* } .-1 }
enum Enum {}

#[diagnostic::do_not_recommend]
// { dg-warning "" "" { target *-*-* } .-1 }
extern "C" {}

#[diagnostic::do_not_recommend]
// { dg-warning "" "" { target *-*-* } .-1 }
fn fun() {}

#[diagnostic::do_not_recommend]
// { dg-warning "" "" { target *-*-* } .-1 }
struct Struct {}

#[diagnostic::do_not_recommend]
// { dg-warning "" "" { target *-*-* } .-1 }
trait Trait {}

#[diagnostic::do_not_recommend]
impl Trait for i32 {}

fn main() {}

