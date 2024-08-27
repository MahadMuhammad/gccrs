// { dg-additional-options "-frust-edition= 2021" }

// Test doesn't fail until monomorphization time, unfortunately.
//@ build-fail

fn main() {
    let _ = async {
        A.first().await.second().await;
    };
}

pub trait First {
    type Second: Second;
    async fn first(self) -> Self::Second;
}

struct A;

impl First for A {
    type Second = A;
    async fn first(self) -> Self::Second {
        A
    }
}

pub trait Second {
    async fn second(self);
}

impl<C> Second for C
where
    C: First,
{
    async fn second(self) {
// { dg-error ".E0733." "" { target *-*-* } .-1 }
        self.first().await.second().await;
    }
}

