// { dg-additional-options "-frust-edition=2018" }

fn foo1(_: &dyn Drop + AsRef<str>) {} // { dg-error ".E0225." "" { target *-*-* } }
// { dg-error ".E0225." "" { target *-*-* } .-1 }

fn foo2(_: &dyn (Drop + AsRef<str>)) {} // { dg-error "" "" { target *-*-* } }

fn foo2_no_space(_: &dyn(Drop + AsRef<str>)) {} // { dg-error "" "" { target *-*-* } }

fn foo3(_: &dyn {Drop + AsRef<str>}) {} // { dg-error ".E0224." "" { target *-*-* } }
// { dg-error ".E0224." "" { target *-*-* } .-1 }
// { dg-error ".E0224." "" { target *-*-* } .-2 }

fn foo4(_: &dyn <Drop + AsRef<str>>) {} // { dg-error "" "" { target *-*-* } }

fn foo5(_: &(dyn Drop + dyn AsRef<str>)) {} // { dg-error ".E0225." "" { target *-*-* } }
// { dg-error ".E0225." "" { target *-*-* } .-1 }

fn main() {}

