fn f1<'a>(_: &'a mut ()) {}

fn f2<P>(_: P, _: ()) {}

fn f3<'a>(x: &'a ((), &'a mut ())) {
    f2(|| x.0, f1(x.1))
// { dg-error ".E0596." "" { target *-*-* } .-1 }
}

fn main() {}

