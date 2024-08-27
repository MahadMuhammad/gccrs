// { dg-additional-options "-frust-edition=2018" }

use core::{
    marker::PhantomPinned,
    mem,
    pin::{pin, Pin},
};

fn main() {
    let mut phantom_pinned = pin!(PhantomPinned);
    mem::take(phantom_pinned.__pointer); // { dg-error ".E0658." "" { target *-*-* } }
}

