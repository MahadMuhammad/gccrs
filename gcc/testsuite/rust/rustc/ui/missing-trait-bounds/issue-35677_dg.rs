//@ run-rustfix
#![allow(dead_code)]
use std::collections::HashSet;
use std::hash::Hash;

fn is_subset<T>(this: &HashSet<T>, other: &HashSet<T>) -> bool {
    this.is_subset(other)
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

fn main() {}

