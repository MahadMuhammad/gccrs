trait HandlerFamily {
    type Target;
}

struct HandlerWrapper<H: HandlerFamily>(H);

impl<H: HandlerFamily> HandlerWrapper<H> {
    pub fn set_handler(&self, handler: &H::Target)
    where
        T: Send + Sync + 'static,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
    {
    }
}

fn main() {}

