//@ check-pass

#![deny(fuzzy_provenance_casts)]
// { dg-warning "" "" { target *-*-* } .-1 }
#![deny(lossy_provenance_casts)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn main() {
    // no warnings emitted since the lints are not activated

    let _dangling = 16_usize as *const u8;

    let x: u8 = 37;
    let _addr: usize = &x as *const u8 as usize;
}

