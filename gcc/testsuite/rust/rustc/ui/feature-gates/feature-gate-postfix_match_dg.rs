// Testing that postfix match doesn't work without enabling the feature

fn main() {
    let val = Some(42);

    val.match { // { dg-error ".E0658." "" { target *-*-* } }
        Some(42) => "the answer to life, the universe, and everything",
        _ => "might be the answer to something"
    };

    // Test that the gate works behind a cfg
    #[cfg(FALSE)]
    val.match { // { dg-error ".E0658." "" { target *-*-* } }
        Some(42) => "the answer to life, the universe, and everything",
        _ => "might be the answer to something"
    };
}

