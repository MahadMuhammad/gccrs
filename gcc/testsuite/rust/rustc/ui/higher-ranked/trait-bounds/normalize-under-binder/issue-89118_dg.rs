trait BufferMut {}
struct Ctx<D>(D);

trait BufferUdpStateContext<B> {}
impl<B: BufferMut, C> BufferUdpStateContext<B> for C {}

trait StackContext
where
    Ctx<()>: for<'a> BufferUdpStateContext<&'a ()>,
{
    type Dispatcher;
}

trait TimerContext {
    type Handler;
}
impl<C> TimerContext for C
where
    C: StackContext,
// { dg-error ".E0277." "" { target *-*-* } .-1 }
{
    type Handler = Ctx<C::Dispatcher>;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

struct EthernetWorker<C>(C)
where
    Ctx<()>: for<'a> BufferUdpStateContext<&'a ()>;
impl<C> EthernetWorker<C> {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

fn main() {}

