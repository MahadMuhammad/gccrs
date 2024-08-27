#![forbid(dead_code)]

#[derive(Debug)]
pub struct Whatever {
    pub field0: (),
    field1: (), // { dg-error "" "" { target *-*-* } }
    field2: (),
    field3: (),
    field4: (),
}

fn main() {}

