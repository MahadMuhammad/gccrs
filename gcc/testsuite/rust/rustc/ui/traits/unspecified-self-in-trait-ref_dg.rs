pub trait Foo<A=Self> {
    fn foo(&self);
}

pub trait Bar<X=usize, A=Self> {
    fn foo(&self);
}

fn main() {
    let a = Foo::lol();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-warning ".E0599." "" { target *-*-* } .-2 }
// { dg-warning ".E0599." "" { target *-*-* } .-3 }
    let b = Foo::<_>::lol();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-warning ".E0599." "" { target *-*-* } .-2 }
// { dg-warning ".E0599." "" { target *-*-* } .-3 }
    let c = Bar::lol();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-warning ".E0599." "" { target *-*-* } .-2 }
// { dg-warning ".E0599." "" { target *-*-* } .-3 }
    let d = Bar::<usize, _>::lol();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-warning ".E0599." "" { target *-*-* } .-2 }
// { dg-warning ".E0599." "" { target *-*-* } .-3 }
    let e = Bar::<usize>::lol();
// { dg-error ".E0393." "" { target *-*-* } .-1 }
// { dg-warning ".E0393." "" { target *-*-* } .-2 }
// { dg-warning ".E0393." "" { target *-*-* } .-3 }
}

