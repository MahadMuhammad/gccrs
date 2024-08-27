use std::sync::atomic::AtomicUsize;

const A: AtomicUsize = AtomicUsize::new(0);
const B: &'static AtomicUsize = &A; // { dg-error ".E0492." "" { target *-*-* } }
static C: &'static AtomicUsize = &A; // { dg-error ".E0492." "" { target *-*-* } }

const NONE: &'static Option<AtomicUsize> = &None;

fn main() {
}

