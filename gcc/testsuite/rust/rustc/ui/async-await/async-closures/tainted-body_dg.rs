// { dg-additional-options "-frust-edition=2021" }

#![feature(async_closure)]

// Don't ICE in ByMove shim builder when MIR body is tainted by writeback errors

fn main() {
    let _ = async || {
        used_fn();
// { dg-error ".E0425." "" { target *-*-* } .-1 }
        0
    };
}

