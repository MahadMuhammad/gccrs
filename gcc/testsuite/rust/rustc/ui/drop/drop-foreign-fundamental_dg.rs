use std::ops::Deref;
use std::pin::Pin;

struct Whatever<T>(T);

impl<T> Deref for Whatever<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct A;

impl Drop for Pin<Whatever<A>> {
// { dg-error ".E0120." "" { target *-*-* } .-1 }
    fn drop(&mut self) {}
}

fn main() {
    let x = Pin::new(Whatever(1.0f32));
}

