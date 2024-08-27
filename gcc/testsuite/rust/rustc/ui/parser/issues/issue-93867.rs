pub struct Entry<'a, K, V> {
    k: &'a mut K,
    v: V,
}

pub fn entry<'a, K, V>() -> Entry<'a K, V> {
//                                  ^ missing comma
// { dg-error "" "" { target *-*-* } .-2 }
    unimplemented!()
}

