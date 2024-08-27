const A: [&str; 1] = { "hello" };
// { dg-error ".E0308." "" { target *-*-* } .-1 }

const B: &[u32] = &{ 1 };
// { dg-error ".E0308." "" { target *-*-* } .-1 }

const C: &&[u32; 1] = &&{ 1 };
// { dg-error ".E0308." "" { target *-*-* } .-1 }

fn main() {}

