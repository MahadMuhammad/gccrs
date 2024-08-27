// { dg-additional-options "-frust-edition=2018" }

async fn hello() { // { help "" "" { target *-*-* } }
    0
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

async fn world() -> () {
    0
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

async fn suggest_await_in_async_fn_return() {
    hello()
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
// { help ".E0308." "" { target *-*-* } .-3 }
// { suggestion ".E0308." "" { target *-*-* } .-4 }
}

fn main() {}

