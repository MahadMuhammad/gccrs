type Foo = Bar;
// { dg-error ".E0412." "" { target *-*-* } .-1 }

fn check(f: impl FnOnce(Foo), val: Foo) {
    f(val);
}

fn main() {}

