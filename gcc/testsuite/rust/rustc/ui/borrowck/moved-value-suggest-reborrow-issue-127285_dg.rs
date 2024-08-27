//@ run-rustfix

#![allow(dead_code)]

struct X(u32);

impl X {
    fn f(&mut self) {
        generic(self);
        self.0 += 1;
// { dg-error ".E0382." "" { target *-*-* } .-1 }
    }
}

fn generic<T>(_x: T) {}

fn main() {}

