#![allow(dead_code)]
#![allow(unused_variables)]

fn bug() {
    macro_rules! m {
        () => {
            _ // { dg-error ".E0121." "" { target *-*-* } }
        };
    }
    struct S<T = m!()>(m!(), T)
    where
        T: Trait<m!()>;
}
trait Trait<T> {}

fn main() {}

