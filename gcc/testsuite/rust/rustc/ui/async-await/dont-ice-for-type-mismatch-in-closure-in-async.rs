// { dg-additional-options "-frust-edition= 2021" }

fn call(_: impl Fn() -> bool) {}

async fn test() {
    call(|| -> Option<()> {
// { dg-error ".E0271." "" { target *-*-* } .-1 }
        if true {
            false
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        }
        true
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    })
}

fn main() {}

