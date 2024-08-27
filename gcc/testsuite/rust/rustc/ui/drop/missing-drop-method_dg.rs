struct DropNoMethod;
impl Drop for DropNoMethod {} // { dg-error ".E0046." "" { target *-*-* } }

fn main() {}

