//@ check-pass

fn main() {
    // Common
    for _ in Some(1) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
    for _ in Ok::<_, ()>(1) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }

    // `Iterator::next` specific
    for _ in [0; 0].iter().next() {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }

    // `Result<impl Iterator, _>`, but function doesn't return `Result`
    for _ in Ok::<_, ()>([0; 0].iter()) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
}

fn _returns_result() -> Result<(), ()> {
    // `Result<impl Iterator, _>`
    for _ in Ok::<_, ()>([0; 0].iter()) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }

    // `Result<impl IntoIterator>`
    for _ in Ok::<_, ()>([0; 0]) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }

    Ok(())
}

fn _by_ref() {
    // Shared refs
    for _ in &Some(1) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
    for _ in &Ok::<_, ()>(1) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }

    // Mutable refs
    for _ in &mut Some(1) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
    for _ in &mut Ok::<_, ()>(1) {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
}

