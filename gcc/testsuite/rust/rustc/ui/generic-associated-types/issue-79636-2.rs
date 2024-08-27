trait SomeTrait {
    type Wrapped<A>: SomeTrait;

    fn f() -> ();
}

fn program<W>() -> ()
where
    W: SomeTrait<Wrapped = W>,
// { dg-error ".E0107." "" { target *-*-* } .-1 }
{
    return W::f();
}

fn main() {}

