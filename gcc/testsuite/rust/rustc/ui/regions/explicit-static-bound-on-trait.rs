struct Hello<'a> {
    value: Box<dyn std::any::Any + 'a>,
// { dg-error ".E0478." "" { target *-*-* } .-1 }
}

impl<'a> Hello<'a> {
    fn new<T: 'a>(value: T) -> Self {
        Self { value: Box::new(value) }
// { dg-error ".E0310." "" { target *-*-* } .-1 }
    }
}

fn main() {}

