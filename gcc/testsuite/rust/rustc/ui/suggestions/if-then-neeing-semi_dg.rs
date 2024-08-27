// { dg-additional-options "-frust-edition=2018" }

fn dummy() -> i32 {
    42
}

fn extra_semicolon() {
    let _ = if true {
// { dg-note ".E0308." "" { target *-*-* } .-1 }
        dummy(); // { dg-note "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
    } else {
        dummy() // { dg-error ".E0308." "" { target *-*-* } }
// { dg-note ".E0308." "" { target *-*-* } .-1 }
    };
}

async fn async_dummy() {}

async fn async_dummy2() {}

async fn async_extra_semicolon_same() {
    let _ = if true {
// { dg-note ".E0308." "" { target *-*-* } .-1 }
        async_dummy(); // { dg-note "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
    } else {
        async_dummy() // { dg-error ".E0308." "" { target *-*-* } }
// { dg-note ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { help ".E0308." "" { target *-*-* } .-3 }
    };
}

async fn async_extra_semicolon_different() {
    let _ = if true {
// { dg-note ".E0308." "" { target *-*-* } .-1 }
        async_dummy(); // { dg-note "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
    } else {
        async_dummy2() // { dg-error ".E0308." "" { target *-*-* } }
// { dg-note ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { help ".E0308." "" { target *-*-* } .-3 }
    };
}

async fn async_different_futures() {
    let _ = if true {
// { dg-note ".E0308." "" { target *-*-* } .-1 }
        async_dummy() // { dg-note "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-2 }
    } else {
        async_dummy2() // { dg-error ".E0308." "" { target *-*-* } }
// { dg-note ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
    };
}

fn main() {}

