fn main() {
    let Some(x) = Some(1) else { // { dg-error ".E0308." "" { target *-*-* } }
        Some(2)
    };
    let Some(x) = Some(1) else { // { dg-error ".E0308." "" { target *-*-* } }
        if 1 == 1 {
            panic!();
        }
    };
    let Some(x) = Some(1) else { Some(2) }; // { dg-error ".E0308." "" { target *-*-* } }

    // Ensure that uninhabited types do not "diverge".
    // This might be relaxed in the future, but when it is,
    // it should be an explicitly wanted decision.
    let Some(x) = Some(1) else { foo::<Uninhabited>() }; // { dg-error ".E0308." "" { target *-*-* } }
}

enum Uninhabited {}

fn foo<T>() -> T {
    panic!()
}

