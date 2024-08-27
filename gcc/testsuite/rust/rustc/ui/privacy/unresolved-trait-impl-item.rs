// { dg-additional-options "-frust-edition=2018" }

trait MyTrait {
    async fn resolved(&self);
    const RESOLVED_WRONG: u8 = 0;
}

impl MyTrait for i32 {
    async fn resolved(&self) {}

    async fn unresolved(&self) {} // { dg-error ".E0407." "" { target *-*-* } }
    async fn RESOLVED_WRONG() {} // { dg-error ".E0324." "" { target *-*-* } }
}

fn main() {}

