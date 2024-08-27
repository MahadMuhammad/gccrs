//@ run-rustfix
#![deny(unsafe_attr_outside_unsafe)]

macro_rules! tt {
    ($e:tt) => {
        #$e
        extern fn foo() {}
    }
}

macro_rules! ident {
    ($e:ident) => {
        #[$e]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
        extern fn bar() {}
    }
}

macro_rules! ident2 {
    ($e:ident, $l:literal) => {
        #[$e = $l]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
        extern fn bars() {}
    }
}

macro_rules! meta {
    ($m:meta) => {
        #[$m]
        extern fn baz() {}
    }
}

macro_rules! meta2 {
    ($m:meta) => {
        #[$m]
        extern fn baw() {}
    }
}

tt!([no_mangle]);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
ident!(no_mangle);
meta!(no_mangle);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
meta2!(export_name = "baw");
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
ident2!(export_name, "bars");

#[no_mangle]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
extern "C" fn one() {}

fn main() {}

