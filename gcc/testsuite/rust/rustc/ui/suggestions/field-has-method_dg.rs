struct Kind;

struct Ty {
    kind: Kind,
}

impl Ty {
    fn kind(&self) -> Kind {
        todo!()
    }
}

struct InferOk<T> {
    value: T,
    predicates: Vec<()>,
}

fn foo(i: InferOk<Ty>) {
    let k = i.kind();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

fn main() {}

