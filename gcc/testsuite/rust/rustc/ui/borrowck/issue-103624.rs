// { dg-additional-options "-frust-edition=2021" }

struct StructA {
    b: StructB,
}

async fn spawn_blocking<T>(f: impl (Fn() -> T) + Send + Sync + 'static) -> T {
    todo!()
}

impl StructA {
    async fn foo(&self) {
        let bar = self.b.bar().await;
        spawn_blocking(move || {
// { dg-error ".E0521." "" { target *-*-* } .-1 }
            self.b;
// { dg-error ".E0507." "" { target *-*-* } .-1 }
        })
        .await;
    }
}

struct StructB {}

impl StructB {
    async fn bar(&self) -> Option<u8> {
        None
    }
}

fn main() {}

