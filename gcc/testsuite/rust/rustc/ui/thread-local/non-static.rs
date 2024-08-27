// Check that #[thread_local] attribute is rejected on non-static items.
#![feature(thread_local)]

#[thread_local]
// { dg-error "" "" { target *-*-* } .-1 }
const A: u32 = 0;

#[thread_local]
// { dg-error "" "" { target *-*-* } .-1 }
fn main() {
    #[thread_local] || {};
// { dg-error "" "" { target *-*-* } .-1 }
}

struct S {
    #[thread_local]
// { dg-error "" "" { target *-*-* } .-1 }
    a: String,
    b: String,
}

#[thread_local]
// Static. OK.
static B: u32 = 0;

extern "C" {
    #[thread_local]
    // Foreign static. OK.
    static C: u32;
}

