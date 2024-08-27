use std::marker;

struct Heap;

struct Vec<T, A = Heap>(
    marker::PhantomData<(T,A)>);

struct HashMap<K, V, S = ()>(marker::PhantomData<(K,V,S)>);

fn main() {
    let _: Vec;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { suggestion ".E0107." "" { target *-*-* } .-2 }

    let _x = (1..10).collect::<HashMap>();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { suggestion ".E0107." "" { target *-*-* } .-2 }

    ().extend::<[(); 0]>({
        fn not_the_extend() {
            let _: Vec;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { suggestion ".E0107." "" { target *-*-* } .-2 }
        }
        []
    });
}

