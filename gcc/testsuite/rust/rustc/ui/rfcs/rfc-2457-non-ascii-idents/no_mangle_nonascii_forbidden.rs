#[no_mangle]
pub fn řųśť() {}  // { dg-error ".E0754." "" { target *-*-* } }

pub struct Foo;

impl Foo {
    #[no_mangle]
    pub fn řųśť() {}  // { dg-error ".E0754." "" { target *-*-* } }
}

trait Bar {
    fn řųśť();
}

impl Bar for Foo {
    #[no_mangle]
    fn řųśť() {}  // { dg-error ".E0754." "" { target *-*-* } }
}

fn main() {}

