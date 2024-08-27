pub enum Value {
    Float(Option<f64>),
}

fn main() {
    let _a = Value::Float( // { dg-error ".E0061." "" { target *-*-* } }
        0,
        None,
        None,
        0,
    );
}

