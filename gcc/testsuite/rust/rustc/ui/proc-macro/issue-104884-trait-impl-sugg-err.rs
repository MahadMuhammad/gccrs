//@ aux-build:issue-104884.rs

use std::collections::BinaryHeap;

#[macro_use]
extern crate issue_104884;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct PriorityQueueEntry<T> {
    value: T,
}

#[derive(PartialOrd, AddImpl)]
// { dg-error ".E0609." "" { target *-*-* } .-1 }
// { dg-error ".E0609." "" { target *-*-* } .-2 }
// { dg-error ".E0609." "" { target *-*-* } .-3 }
// { dg-error ".E0609." "" { target *-*-* } .-4 }
// { dg-error ".E0609." "" { target *-*-* } .-5 }

struct PriorityQueue<T>(BinaryHeap<PriorityQueueEntry<T>>);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
fn main() {}

