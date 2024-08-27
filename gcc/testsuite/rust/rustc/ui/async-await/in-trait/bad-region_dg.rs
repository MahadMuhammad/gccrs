//@ aux-build:bad-region.rs
// { dg-additional-options "-frust-edition=2021" }

#![allow(async_fn_in_trait)]

extern crate bad_region as jewel;

use jewel::BleRadio;

pub struct Radio {}

impl BleRadio for Radio {
// { dg-error ".E0726." "" { target *-*-* } .-1 }
    async fn transmit(&mut self) {}
}

fn main() {}

