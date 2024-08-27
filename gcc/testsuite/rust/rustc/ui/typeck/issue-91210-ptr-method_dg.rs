// Regression test for issue #91210.

//@ run-rustfix

#![allow(unused)]

struct Foo { read: i32 }

unsafe fn blah(x: *mut Foo) {
    x.read = 4;
// { dg-error ".E0615." "" { target *-*-* } .-1 }
// { help ".E0615." "" { target *-*-* } .-2 }
}

fn main() {}

