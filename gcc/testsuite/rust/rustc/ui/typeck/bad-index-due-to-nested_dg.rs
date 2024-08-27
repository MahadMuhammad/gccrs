use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Index;

struct HashMap<K, V>(PhantomData<(K, V)>);

impl<K, V> Index<&K> for HashMap<K, V>
where
    K: Hash,
    V: Copy,
{
    type Output = V;

    fn index(&self, k: &K) -> &V {
        todo!()
    }
}

fn index<'a, K, V>(map: &'a HashMap<K, V>, k: K) -> &'a V {
    map[k]
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
// { dg-error ".E0308." "" { target *-*-* } .-4 }
}

fn main() {}

