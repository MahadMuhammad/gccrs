// Regression test for #93927: suggested trait bound for T should be Eq, not PartialEq
struct MyType<T>(T);

impl<T> PartialEq for MyType<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

fn cond<T: PartialEq>(val: MyType<T>) -> bool {
    val == val
// { dg-error ".E0369." "" { target *-*-* } .-1 }
}

fn main() {
    cond(MyType(0));
}

