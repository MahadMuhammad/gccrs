#![doc = in_root!()] // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
#![doc = in_mod!()] // { dg-error "" "" { target *-*-* } }
#![doc = in_mod_escape!()] // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
#![doc = in_block!()] // { dg-error "" "" { target *-*-* } }

#[doc = in_root!()] // { dg-error "" "" { target *-*-* } }
#[doc = in_mod!()] // { dg-error "" "" { target *-*-* } }
#[doc = in_mod_escape!()] // { dg-error "" "" { target *-*-* } }
#[doc = in_block!()] // { dg-error "" "" { target *-*-* } }
fn before() {
    #![doc = in_root!()] // { dg-error "" "" { target *-*-* } }
    #![doc = in_mod!()] // { dg-error "" "" { target *-*-* } }
    #![doc = in_mod_escape!()] // { dg-error "" "" { target *-*-* } }
    #![doc = in_block!()] // { dg-error "" "" { target *-*-* } }
}

macro_rules! in_root { () => { "" } }

#[doc = in_mod!()] // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
mod macros_stay {
    #![doc = in_mod!()] // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }

    macro_rules! in_mod { () => { "" } }

    #[doc = in_mod!()] // OK
    fn f() {
        #![doc = in_mod!()] // OK
    }
}

#[macro_use]
#[doc = in_mod_escape!()] // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
mod macros_escape {
    #![doc = in_mod_escape!()] // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }

    macro_rules! in_mod_escape { () => { "" } }

    #[doc = in_mod_escape!()] // OK
    fn f() {
        #![doc = in_mod_escape!()] // OK
    }
}

#[doc = in_block!()] // { dg-error "" "" { target *-*-* } }
fn block() {
    #![doc = in_block!()] // { dg-error "" "" { target *-*-* } }

    macro_rules! in_block { () => { "" } }

    #[doc = in_block!()] // OK
    fn f() {
        #![doc = in_block!()] // OK
    }
}

#[doc = in_root!()] // OK
#[doc = in_mod!()] // { dg-error "" "" { target *-*-* } }
#[doc = in_mod_escape!()] // OK
#[doc = in_block!()] // { dg-error "" "" { target *-*-* } }
fn after() {
    #![doc = in_root!()] // OK
    #![doc = in_mod!()] // { dg-error "" "" { target *-*-* } }
    #![doc = in_mod_escape!()] // OK
    #![doc = in_block!()] // { dg-error "" "" { target *-*-* } }
}

fn main() {}

