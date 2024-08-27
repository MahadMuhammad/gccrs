fn f() { }
struct S(Box<dyn FnMut() + Sync>);
pub static C: S = S(f); // { dg-error ".E0308." "" { target *-*-* } }


fn g() { }
type T = Box<dyn FnMut() + Sync>;
pub static D: T = g; // { dg-error ".E0308." "" { target *-*-* } }

fn main() {}

