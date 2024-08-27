//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver
//@[next] check-pass

trait MyFn<T> {
    type Output;
}

trait Callback<A>: MyFn<A, Output = Self::Ret> {
    type Ret;
}

impl<A, F: MyFn<A>> Callback<A> for F {
    type Ret = F::Output;
}

struct Thing;
trait Trait {}
impl Trait for Thing {}

trait ChannelSender {
    type CallbackArg;

    fn autobatch<F>(self) -> impl Trait
    where
        F: Callback<Self::CallbackArg>;
// { dg-error "" "" { target *-*-* } .-1 }
}

struct Sender;

impl ChannelSender for Sender {
    type CallbackArg = i32;

    fn autobatch<F>(self) -> impl Trait
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
    where
        F: Callback<Self::CallbackArg>,
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    {
        Thing
    }
}

fn main() {}

