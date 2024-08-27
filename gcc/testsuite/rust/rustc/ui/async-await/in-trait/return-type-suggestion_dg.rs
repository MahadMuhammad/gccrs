// { dg-additional-options "-frust-edition= 2021" }


trait A {
    async fn e() {
        Ok(())
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
}

fn main() {}

