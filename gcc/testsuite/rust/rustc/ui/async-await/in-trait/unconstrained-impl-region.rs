// { dg-additional-options "-frust-edition= 2021" }

pub(crate) trait Inbox<M> {
    async fn next(self) -> M;
}

pub(crate) trait Actor: Sized {
    type Message;

    async fn on_mount(self, _: impl Inbox<Self::Message>);
}

impl<'a> Actor for () {
// { dg-error ".E0207." "" { target *-*-* } .-1 }
    type Message = &'a ();
    async fn on_mount(self, _: impl Inbox<&'a ()>) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

