struct Checked<const F: fn(usize) -> bool>;
// { dg-error "" "" { target *-*-* } .-1 }
fn not_one(val: usize) -> bool { val != 1 }
const _: Checked<not_one> = Checked::<not_one>;
fn main() {}

