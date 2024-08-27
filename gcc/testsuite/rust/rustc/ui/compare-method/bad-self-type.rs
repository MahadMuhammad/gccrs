use std::future::Future;
use std::task::{Context, Poll};

fn main() {}

struct MyFuture {}

impl Future for MyFuture {
    type Output = ();
    fn poll(self, _: &mut Context<'_>) -> Poll<()> {
// { dg-error ".E0053." "" { target *-*-* } .-1 }
        todo!()
    }
}

trait T {
    fn foo(self);
    fn bar(self) -> Option<()>;
}

impl T for MyFuture {
    fn foo(self: Box<Self>) {}
// { dg-error ".E0053." "" { target *-*-* } .-1 }
    fn bar(self) {}
// { dg-error ".E0053." "" { target *-*-* } .-1 }
}

