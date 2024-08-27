//@ revisions: allowed disallowed
//@[allowed] check-pass

#![feature(if_let_guard, let_chains)]
#![cfg_attr(allowed, allow(irrefutable_let_patterns))]
#![cfg_attr(disallowed, deny(irrefutable_let_patterns))]

use std::ops::Range;

fn main() {
    let opt = Some(None..Some(1));

    if let first = &opt && let Some(ref second) = first && let None = second.start {}
// { dg-error "" "" { target *-*-* } .-1 }

    // No lint as the irrefutable pattern is surrounded by other stuff
    if 4 * 2 == 0 && let first = &opt && let Some(ref second) = first && let None = second.start {}

    if let first = &opt && let (a, b) = (1, 2) {}
// { dg-error "" "" { target *-*-* } .-1 }

    if let first = &opt && let Some(ref second) = first && let None = second.start && let v = 0 {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    if let Some(ref first) = opt && let second = first && let _third = second {}
// { dg-error "" "" { target *-*-* } .-1 }

    if let Range { start: local_start, end: _ } = (None..Some(1)) && let None = local_start {}
// { dg-error "" "" { target *-*-* } .-1 }

    if let (a, b, c) = (Some(1), Some(1), Some(1)) && let None = Some(1) {}
// { dg-error "" "" { target *-*-* } .-1 }

    if let first = &opt && let None = Some(1) {}
// { dg-error "" "" { target *-*-* } .-1 }

    if let Some(ref first) = opt
        && let Range { start: local_start, end: _ } = first
        && let None = local_start {
    }

    match opt {
        Some(ref first) if let second = first && let _third = second && let v = 4 + 4 => {},
// { dg-error "" "" { target *-*-* } .-1 }
        _ => {}
    }

    // No error about leading irrefutable patterns: the expr on the rhs might
    // use the bindings created by the match.
    match opt {
        Some(ref first) if let Range { start: local_start, end: _ } = first
            && let None = local_start => {},
        _ => {}
    }

    match opt {
        Some(ref first) if let Range { start: Some(_), end: local_end } = first
            && let v = local_end && let w = v => {},
// { dg-error "" "" { target *-*-* } .-1 }
        _ => {}
    }

    // No error, despite the prefix being irrefutable: moving out could change the behaviour,
    // due to possible side effects of the operation.
    while let first = &opt && let Some(ref second) = first && let None = second.start {}

    while let first = &opt && let (a, b) = (1, 2) {}
// { dg-error "" "" { target *-*-* } .-1 }

    while let Some(ref first) = opt && let second = first && let _third = second {}
// { dg-error "" "" { target *-*-* } .-1 }

    while let Some(ref first) = opt
        && let Range { start: local_start, end: _ } = first
        && let None = local_start {
    }
}

