// { dg-additional-options "-frust-edition=2021" }

// Test that we do not suggest `.await` when it doesn't make sense.

struct A;

impl A {
    fn test(&self) -> i32 {
        1
    }
}

async fn foo() -> A {
    A
}

async fn async_main() {
    let x: u32 = foo().test();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

fn main() {
    let _ = async_main();
}

