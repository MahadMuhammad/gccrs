use std::cell::RefCell;

pub struct MessageListeners<'a> {
    listeners: RefCell<Vec<Box<dyn FnMut(()) + 'a>>>,
}

pub trait MessageListenersInterface {
    fn listeners<'c>(&'c self) -> &'c MessageListeners<'c>;
}

impl<'a> MessageListenersInterface for MessageListeners<'a> {
    fn listeners<'b>(&'b self) -> &'a MessageListeners<'b> {
// { dg-error ".E0491." "" { target *-*-* } .-1 }
        self
    }
}

fn main() {}

