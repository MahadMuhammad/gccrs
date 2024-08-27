trait T {
    fn foo(&self);

pub(crate) struct Bar<T>();

impl T for Bar<usize> {

fn foo(&self) {}
}

fn main() {} // { dg-error "" "" { target *-*-* } }

