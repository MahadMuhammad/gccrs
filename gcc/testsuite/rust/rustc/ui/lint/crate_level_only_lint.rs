#![deny(uncommon_codepoints, unused_attributes)]

mod foo {
#![allow(uncommon_codepoints)]
// { dg-error "" "" { target *-*-* } .-1 }

#[allow(uncommon_codepoints)]
// { dg-error "" "" { target *-*-* } .-1 }
const BAR: f64 = 0.000001;

}

#[allow(uncommon_codepoints)]
// { dg-error "" "" { target *-*-* } .-1 }
fn main() {
}

