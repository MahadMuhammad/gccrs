//@ ignore-apple: cycle error does not appear on apple

use std::sync::Mutex;

enum Foo { X(Mutex<Option<Foo>>) }
// { dg-error ".E0391." "" { target *-*-* } .-1 }
// { dg-error ".E0391." "" { target *-*-* } .-2 }

impl Foo { fn bar(self) {} }

fn main() {}

