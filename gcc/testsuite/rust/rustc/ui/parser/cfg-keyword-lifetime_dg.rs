// Disallow `'keyword` even in cfg'd code.

#[cfg(any())]
fn hello() -> &'ref () {}
// { dg-error "" "" { target *-*-* } .-1 }

macro_rules! macro_invocation {
    ($i:item) => {}
}
macro_invocation! {
    fn hello() -> &'ref () {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

