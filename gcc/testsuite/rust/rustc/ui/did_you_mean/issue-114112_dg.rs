enum E<T> {
    A(T)
}

fn main() {
    match E::<i32>::A(1) {
        E<i32>::A(v) => { // { dg-error "" "" { target *-*-* } }
            println!("{v:?}");
        },
    }
}

