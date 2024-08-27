//@ build-fail
#![feature(rustc_attrs)]

//   O --> G --> C --> A
//     \     \     \-> B
//     |     |-> F --> D
//     |           \-> E
//     |-> N --> J --> H
//           \     \-> I
//           |-> M --> K
//                 \-> L

#[rustc_dump_vtable]
trait A {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_a(&self) {}
}

#[rustc_dump_vtable]
trait B {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_b(&self) {}
}

#[rustc_dump_vtable]
trait C: A + B {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_c(&self) {}
}

#[rustc_dump_vtable]
trait D {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_d(&self) {}
}

#[rustc_dump_vtable]
trait E {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_e(&self) {}
}

#[rustc_dump_vtable]
trait F: D + E {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_f(&self) {}
}

#[rustc_dump_vtable]
trait G: C + F {
    fn foo_g(&self) {}
}

#[rustc_dump_vtable]
trait H {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_h(&self) {}
}

#[rustc_dump_vtable]
trait I {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_i(&self) {}
}

#[rustc_dump_vtable]
trait J: H + I {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_j(&self) {}
}

#[rustc_dump_vtable]
trait K {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_k(&self) {}
}

#[rustc_dump_vtable]
trait L {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_l(&self) {}
}

#[rustc_dump_vtable]
trait M: K + L {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_m(&self) {}
}

#[rustc_dump_vtable]
trait N: J + M {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_n(&self) {}
}

#[rustc_dump_vtable]
trait O: G + N {
// { dg-error "" "" { target *-*-* } .-1 }
    fn foo_o(&self) {}
}

struct S;

impl A for S {}
impl B for S {}
impl C for S {}
impl D for S {}
impl E for S {}
impl F for S {}
impl G for S {}
impl H for S {}
impl I for S {}
impl J for S {}
impl K for S {}
impl L for S {}
impl M for S {}
impl N for S {}
impl O for S {}

macro_rules! monomorphize_vtable {
    ($trait:ident) => {{
        fn foo(_ : &dyn $trait) {}
        foo(&S);
    }}
}

fn main() {
    monomorphize_vtable!(O);

    monomorphize_vtable!(A);
    monomorphize_vtable!(B);
    monomorphize_vtable!(C);
    monomorphize_vtable!(D);
    monomorphize_vtable!(E);
    monomorphize_vtable!(F);
    monomorphize_vtable!(H);
    monomorphize_vtable!(I);
    monomorphize_vtable!(J);
    monomorphize_vtable!(K);
    monomorphize_vtable!(L);
    monomorphize_vtable!(M);
    monomorphize_vtable!(N);
}

