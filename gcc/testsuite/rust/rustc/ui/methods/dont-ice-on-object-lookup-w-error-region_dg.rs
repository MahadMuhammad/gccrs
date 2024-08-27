// Fix for issue: #122914

use std::future::Future;
use std::pin::Pin;

fn project(x: Pin<&'missing mut dyn Future<Output = ()>>) {
// { dg-error ".E0261." "" { target *-*-* } .-1 }
    let _ = x.poll(todo!());
}

fn main() {}

