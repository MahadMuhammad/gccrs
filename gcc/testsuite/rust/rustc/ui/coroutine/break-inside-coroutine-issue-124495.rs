// { dg-additional-options "-frust-edition= 2024" }
//@ compile-flags: -Z unstable-options

#![feature(gen_blocks)]
#![feature(async_closure)]

async fn async_fn() {
    break; // { dg-error ".E0267." "" { target *-*-* } }
}

gen fn gen_fn() {
    break; // { dg-error ".E0267." "" { target *-*-* } }
}

async gen fn async_gen_fn() {
    break; // { dg-error ".E0267." "" { target *-*-* } }
}

fn main() {
    let _ = async { break; }; // { dg-error ".E0267." "" { target *-*-* } }

    let _ = async || { break; }; // { dg-error ".E0267." "" { target *-*-* } }

    let _ = gen { break; }; // { dg-error ".E0267." "" { target *-*-* } }

    let _ = async gen { break; }; // { dg-error ".E0267." "" { target *-*-* } }
}

