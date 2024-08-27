//@ aux-build:cfged_out.rs

extern crate cfged_out;

fn main() {
    // There is no uwu at this path - no diagnostic.
    cfged_out::uwu(); // { dg-error ".E0425." "" { target *-*-* } }
// { dg-note ".E0425." "" { target *-*-* } .-1 }

    // It does exist here - diagnostic.
    cfged_out::inner::uwu(); // { dg-error ".E0425." "" { target *-*-* } }
// { dg-note ".E0425." "" { target *-*-* } .-1 }
// { dg-note ".E0425." "" { target *-*-* } .-2 }
// { dg-note ".E0425." "" { target *-*-* } .-3 }

    // The module isn't found - we would like to get a diagnostic, but currently don't due to
    // the awkward way the resolver diagnostics are currently implemented.
    cfged_out::inner::doesnt_exist::hello(); // { dg-error ".E0433." "" { target *-*-* } }
// { dg-note ".E0433." "" { target *-*-* } .-1 }
// { dg-note ".E0433." "" { target *-*-* } .-2 }
// { dg-note ".E0433." "" { target *-*-* } .-3 }

    // It should find the one in the right module, not the wrong one.
    cfged_out::inner::right::meow(); // { dg-error ".E0425." "" { target *-*-* } }
// { dg-note ".E0425." "" { target *-*-* } .-1 }
// { dg-note ".E0425." "" { target *-*-* } .-2 }
// { dg-note ".E0425." "" { target *-*-* } .-3 }

    // Exists in the crate root - diagnostic.
    cfged_out::vanished(); // { dg-error ".E0425." "" { target *-*-* } }
// { dg-note ".E0425." "" { target *-*-* } .-1 }
// { dg-note ".E0425." "" { target *-*-* } .-2 }
// { dg-note ".E0425." "" { target *-*-* } .-3 }
}

