struct Bar;
trait PrivTr {}

pub trait PubTr {
    type Alias1;
    type Alias2;
}

impl PubTr for u8 {
    type Alias1 = Bar; // { dg-error ".E0446." "" { target *-*-* } }
    type Alias2 = Box<dyn PrivTr>; // { dg-error ".E0446." "" { target *-*-* } }
}

fn main() {}

