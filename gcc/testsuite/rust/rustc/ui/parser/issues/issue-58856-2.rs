struct Empty;

trait Howness {}

impl Howness for () {
    fn how_are_you(&self -> Empty {
        Empty
    }
} // { dg-error "" "" { target *-*-* } }

fn main() {}

