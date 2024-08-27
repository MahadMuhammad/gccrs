// { dg-additional-options "-frust-edition= 2021" }
#![feature(effects)] // { dg-warning "" "" { target *-*-* } }

trait MyTrait {}

impl MyTrait for i32 {
    async const fn bar(&self) {
// { dg-error ".E0407." "" { target *-*-* } .-1 }
// { dg-error ".E0407." "" { target *-*-* } .-2 }
// { dg-error ".E0407." "" { target *-*-* } .-3 }
// { dg-error ".E0407." "" { target *-*-* } .-4 }
        main8().await;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    }
}

fn main() {}

