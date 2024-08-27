#![feature(lazy_type_alias)]
#![allow(incomplete_features)]

type Alias = Local;
struct Local;

impl Alias { fn method() {} } // { dg-error ".E0592." "" { target *-*-* } }
impl Local { fn method() {} }

fn main() {}

