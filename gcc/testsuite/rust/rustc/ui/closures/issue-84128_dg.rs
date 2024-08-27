// test for issue 84128
// missing suggestion for similar ADT type with diffetent generic parameter
// on closure ReturnNoExpression

struct Foo<T>(T);

fn main() {
    || {
        if false {
            return Foo(0);
        }

        Foo(())
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    };
}

