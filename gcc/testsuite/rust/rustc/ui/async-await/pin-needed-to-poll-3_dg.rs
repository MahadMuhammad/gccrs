use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};


struct FutureWrapper<F> {
    fut: F,
}

impl<F> Future for FutureWrapper<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = self.fut.poll(cx);
// { dg-error ".E0599." "" { target *-*-* } .-1 }
        res
    }
}

fn main() {}

