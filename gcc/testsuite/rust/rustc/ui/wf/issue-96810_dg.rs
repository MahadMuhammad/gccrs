struct S<T: Tr>(T::Assoc);

trait Tr {
    type Assoc;
}

struct Hoge<K> {
    s: S<K>, // { dg-error ".E0277." "" { target *-*-* } }
    a: u32,
}

fn main() {}

