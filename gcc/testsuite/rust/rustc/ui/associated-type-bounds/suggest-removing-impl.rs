trait Tr {
    type Assoc: impl Sized;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    fn fn_with_generics<T>()
    where
        T: impl Sized
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    {}
}

fn main() {}

