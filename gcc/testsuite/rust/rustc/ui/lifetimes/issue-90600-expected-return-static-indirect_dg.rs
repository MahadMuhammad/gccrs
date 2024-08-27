use std::cell::RefCell;
use std::io::Read;

fn main() {}

fn inner(mut foo: &[u8]) {
    let refcell = RefCell::new(&mut foo);
// { dg-error ".E0597." "" { target *-*-* } .-1 }
    let read = &refcell as &RefCell<dyn Read>;
// { dg-error "" "" { target *-*-* } .-1 }

    read_thing(read);
}

fn read_thing(refcell: &RefCell<dyn Read>) {}

