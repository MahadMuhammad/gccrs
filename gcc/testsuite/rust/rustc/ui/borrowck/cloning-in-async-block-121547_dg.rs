// { dg-additional-options "-frust-edition=2021" }

async fn clone_async_block(value: String) {
    for _ in 0..10 {
        async { // { dg-error ".E0382." "" { target *-*-* } }
            drop(value);
// { help "" "" { target *-*-* } .-1 }
        }.await
    }
}
fn main() {}

