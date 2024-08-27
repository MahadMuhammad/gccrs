enum Ty {
    Unit,
    List(Box<Ty>),
}

fn foo(x: Ty) -> Ty {
    match x {
        Ty::Unit => Ty::Unit,
        Ty::List(elem) => foo(elem),
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    }
}

fn main() {}

