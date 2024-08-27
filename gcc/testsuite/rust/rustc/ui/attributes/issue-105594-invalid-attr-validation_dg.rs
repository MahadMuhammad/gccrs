// This checks that the attribute validation ICE in issue #105594 doesn't
// recur.
//
//@ ignore-thumbv8m.base-none-eabi
#![feature(cmse_nonsecure_entry)]

fn main() {}

#[track_caller] // { dg-error ".E0739." "" { target *-*-* } }
static _A: () = ();

#[cmse_nonsecure_entry] // { dg-error ".E0775." "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-2 }

