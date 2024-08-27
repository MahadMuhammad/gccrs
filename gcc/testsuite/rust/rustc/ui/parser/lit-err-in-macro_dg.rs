macro_rules! f {
    ($abi:literal) => {
        extern $abi fn f() {}
    }
}

f!("Foo"__);
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

