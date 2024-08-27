#![deny(unreachable_patterns)]

// We wrap patterns in a tuple because top-level or-patterns were special-cased.
#[rustfmt::skip]
fn main() {
    match (0u8,) {
        (1 | 2,) => {}
        (1,) => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
    match (0u8,) {
        (1 | 2,) => {}
        (2,) => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
    match (0u8,) {
        (1,) => {}
        (2,) => {}
        (1 | 2,) => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
    match (0u8, 0u8) {
        (1 | 2, 3 | 4) => {}
        (1, 3) => {}     // { dg-error "" "" { target *-*-* } }
        (1, 4) => {}     // { dg-error "" "" { target *-*-* } }
        (2, 4) => {}     // { dg-error "" "" { target *-*-* } }
        (2 | 1, 4) => {} // { dg-error "" "" { target *-*-* } }
        (1, 5 | 6) => {}
        (1, 4 | 5) => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
    match (0u8, 0u8, 0u8) {
        (0, 0, 0) => {}
        (0, 0 | 1, 0) => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
    match (true, true) {
        (false | true, false | true) => (),
    }
    match (Some(0u8),) {
        (None | Some(1 | 2),) => {}
        (Some(1),) => {} // { dg-error "" "" { target *-*-* } }
        (None,) => {}    // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
    match ((0u8,),) {
        ((1 | 2,) | (3 | 4,),) => {}
        ((1..=4,),) => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }

    match (0,) {
        (1 | 1,) => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
    match 0 {
        (0 | 1) | 1 => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
    match 0 {
        // We get two errors because recursive or-pattern expansion means we don't notice the two
        // errors span a whole pattern. This could be better but doesn't matter much
        0 | (0 | 0) => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        _ => {}
    }
    match None {
        // There is only one error that correctly points to the whole subpattern
        Some(0) |
            Some( // { dg-error "" "" { target *-*-* } }
                0 | 0) => {}
        _ => {}
    }
    match [0; 2] {
        [0
            | 0 // { dg-error "" "" { target *-*-* } }
        , 0
            | 0] => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
    match (true, 0) {
        (true, 0 | 0) => {} // { dg-error "" "" { target *-*-* } }
        (_, 0 | 0) => {} // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
    match &[][..] {
        [0] => {}
        [0, _] => {}
        [0, _, _] => {}
        [1, ..] => {}
        [1 // { dg-error "" "" { target *-*-* } }
            | 2, ..] => {}
        _ => {}
    }
    match &[][..] {
        [true] => {}
        [true | false, ..] => {}
        _ => {}
    }
    match &[][..] {
        [false] => {}
        [true, ..] => {}
        [true // { dg-error "" "" { target *-*-* } }
            | false, ..] => {}
        _ => {}
    }
    match (true, None) {
        (true, Some(_)) => {}
        (false, Some(true)) => {}
        (true | false, None | Some(true // { dg-error "" "" { target *-*-* } }
                                   | false)) => {}
    }
    macro_rules! t_or_f {
        () => {
            (true // { dg-error "" "" { target *-*-* } }
            | false)
        };
    }
    match (true, None) {
        (true, Some(_)) => {}
        (false, Some(true)) => {}
        (true | false, None | Some(t_or_f!())) => {}
    }
    match Some(0) {
        Some(0) => {}
        Some(0 // { dg-error "" "" { target *-*-* } }
             | 1) => {}
        _ => {}
    }

    // A subpattern that is only unreachable in one branch is overall reachable.
    match (true, true) {
        (true, true) => {}
        (false | true, false | true) => {}
    }
    match (true, true) {
        (true, true) => {}
        (false, false) => {}
        (false | true, false | true) => {}
    }
    // https://github.com/rust-lang/rust/issues/76836
    match None {
        Some(false) => {}
        None | Some(true
                | false) => {} // { dg-error "" "" { target *-*-* } }
    }

    // A subpattern that is unreachable in all branches is overall unreachable.
    match (true, true) {
        (false, true) => {}
        (true, true) => {}
        (false | true, false
            | true) => {} // { dg-error "" "" { target *-*-* } }
    }
    match (true, true) {
        (true, false) => {}
        (true, true) => {}
        (false
            | true, // { dg-error "" "" { target *-*-* } }
            false | true) => {}
    }
    match (true, true) {
        (x, y)
            | (y, x) => {} // { dg-error "" "" { target *-*-* } }
    }
}

fn unreachable_in_param((_ | (_, _)): (bool, bool)) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn unreachable_in_binding() {
    let bool_pair = (true, true);
    let bool_option = Some(true);

    let (_ | (_, _)) = bool_pair;
// { dg-error "" "" { target *-*-* } .-1 }
    for (_ | (_, _)) in [bool_pair] {}
// { dg-error "" "" { target *-*-* } .-1 }

    let (Some(_) | Some(true)) = bool_option else { return };
// { dg-error "" "" { target *-*-* } .-1 }
    if let Some(_) | Some(true) = bool_option {}
// { dg-error "" "" { target *-*-* } .-1 }
    while let Some(_) | Some(true) = bool_option {}
// { dg-error "" "" { target *-*-* } .-1 }
}

