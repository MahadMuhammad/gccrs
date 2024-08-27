// Found in a crater run on #118553

pub trait Debug {}

pub trait Service {
    type Input;
    type Output;
    type Error;
}

pub struct ServiceChain<P, S> {
    prev: P,
    service: S,
}
impl<P: Service, S: Service<Input = P::Output>> Service for ServiceChain<P, S>
where
    P::Error: 'static,
    S::Error: 'static,
{
    type Input = P::Input;
    type Output = S::Output;
    type Error = ();
}

pub struct ServiceChainBuilder<P: Service, S: Service<Input = P::Output>> {
    chain: ServiceChain<P, S>,
}
impl<P: Service, S: Service<Input = P::Output>> ServiceChainBuilder<P, S> {
    pub fn next<NS: Service<Input = S::Output>>(
        self,
    ) -> ServiceChainBuilder<ServiceChain<P, S>, NS> {
// { dg-error ".E0310." "" { target *-*-* } .-1 }
// { dg-error ".E0310." "" { target *-*-* } .-2 }
// { dg-error ".E0310." "" { target *-*-* } .-3 }
        panic!();
    }
}

fn main() {}

