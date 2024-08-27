// Regression test for #82126. Checks that mismatched lifetimes and types are
// properly handled.

// { dg-additional-options "-frust-edition=2018" }

use std::sync::Mutex;

struct MarketMultiplier {}

impl MarketMultiplier {
    fn buy(&mut self) -> &mut usize {
        todo!()
    }
}

async fn buy_lock(coroutine: &Mutex<MarketMultiplier>) -> LockedMarket<'_> {
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
// { dg-error ".E0107." "" { target *-*-* } .-3 }
// { dg-error ".E0107." "" { target *-*-* } .-4 }
    LockedMarket(coroutine.lock().unwrap().buy())
}

struct LockedMarket<T>(T);

fn main() {}

