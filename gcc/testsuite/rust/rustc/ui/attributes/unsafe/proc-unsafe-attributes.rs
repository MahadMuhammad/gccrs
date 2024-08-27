#[unsafe(proc_macro)]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
pub fn a() {}


#[unsafe(proc_macro_derive(Foo))]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
pub fn b() {}

#[proc_macro_derive(unsafe(Foo))]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
pub fn c() {}

#[unsafe(proc_macro_attribute)]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
pub fn d() {}

#[unsafe(allow(dead_code))]
// { dg-error "" "" { target *-*-* } .-1 }
pub fn e() {}

#[unsafe(allow(unsafe(dead_code)))]
// { dg-error ".E0452." "" { target *-*-* } .-1 }
// { dg-error ".E0452." "" { target *-*-* } .-2 }
// { dg-error ".E0452." "" { target *-*-* } .-3 }
// { dg-error ".E0452." "" { target *-*-* } .-4 }
// { dg-error ".E0452." "" { target *-*-* } .-5 }
// { dg-error ".E0452." "" { target *-*-* } .-6 }
pub fn f() {}

fn main() {}

