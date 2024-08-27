fn foo<const N: i32>() -> i32 {
    N
}

const fn bar(n: i32, m: i32) -> i32 {
    n
}

const fn baz() -> i32 {
    1
}

const FOO: i32 = 3;

fn main() {
    foo::<baz()>(); // { dg-error ".E0747." "" { target *-*-* } }
// { dg-error ".E0747." "" { target *-*-* } .-2 }
    foo::<bar(bar(1, 1), bar(1, 1))>(); // { dg-error "" "" { target *-*-* } }
    foo::<bar(1, 1)>(); // { dg-error "" "" { target *-*-* } }
    foo::<bar(FOO, 2)>(); // { dg-error "" "" { target *-*-* } }
}

