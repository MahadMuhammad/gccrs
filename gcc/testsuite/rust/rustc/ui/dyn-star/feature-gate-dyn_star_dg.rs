// Feature gate test for dyn_star

/// dyn* is not necessarily the final surface syntax (if we have one at all),
/// but for now we will support it to aid in writing tests independently.
pub fn dyn_star_parameter(_: &dyn* Send) {
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

fn main() {}

