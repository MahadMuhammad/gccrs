// { dg-additional-options "-frust-edition=2018" }

fn take_u32(_x: u32) {}

async fn make_u32() -> u32 {
    22
}

#[allow(unused)]
async fn suggest_await_in_async_fn() {
    let x = make_u32();
    take_u32(x)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
// { suggestion ".E0308." "" { target *-*-* } .-3 }
}

async fn dummy() {}

#[allow(unused)]
async fn suggest_await_in_async_fn_return() {
    dummy()
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
// { help ".E0308." "" { target *-*-* } .-3 }
// { suggestion ".E0308." "" { target *-*-* } .-4 }
}

#[allow(unused)]
async fn suggest_await_on_if() {
    let _x = if true {
        dummy()
// { help "" "" { target *-*-* } .-1 }
    } else {
        dummy().await
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    };
}

#[allow(unused)]
async fn suggest_await_on_previous_match_arms() {
    let _x = match 0usize {
        0 => dummy(), // { help "" "" { target *-*-* } }
        1 => dummy(),
        2 => dummy().await,
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    };
}

#[allow(unused)]
async fn suggest_await_on_match_expr() {
    let _x = match dummy() { // { help "" "" { target *-*-* } }
        () => {} // { dg-error ".E0308." "" { target *-*-* } }
    };
}

async fn dummy_result() -> Result<(), ()> {
    Ok(())
}

#[allow(unused)]
async fn suggest_await_in_generic_pattern() {
    match dummy_result() {
// { help "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { suggestion "" "" { target *-*-* } .-3 }
        Ok(_) => {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        Err(_) => {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
}

// Issue #126903
async fn do_async() {}
fn dont_suggest_awaiting_closure_patterns() {
    Some(do_async()).map(|()| {});
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

