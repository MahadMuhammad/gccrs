//@ check-fail

#[derive(PartialEq, Eq)]
pub enum Value {
    Boolean(Option<bool>),
    Float(Option<f64>), // { dg-error ".E0277." "" { target *-*-* } }
}

fn main() {
    let a = Value::Float(Some(f64::NAN));
    assert!(a == a);
}

