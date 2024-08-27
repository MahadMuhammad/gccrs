//@ compile-flags: --edition 2024 -Zunstable-options

#![feature(gen_blocks)]

async gen fn async_gen_fn() -> i32 { 0 }
// { dg-error ".E0308." "" { target *-*-* } .-1 }

gen fn gen_fn() -> i32 { 0 }
// { dg-error ".E0308." "" { target *-*-* } .-1 }

fn async_gen_block() {
    async gen { yield (); 1 };
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn gen_block() {
    gen { yield (); 1 };
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

