#[patchable_function_entry(prefix_nops = 1, entry_nops = 1)]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
fn main() {}

