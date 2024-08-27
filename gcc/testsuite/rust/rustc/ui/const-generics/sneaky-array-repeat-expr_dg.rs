trait Trait<const N: usize> {
    const Assoc: usize;
}

impl<const N: usize> Trait<N> for () {
    const Assoc: usize = 1;
}


pub const fn foo<const N: usize>() where (): Trait<N> {
    let bar = [(); <()>::Assoc];
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

trait Trait2<const N: usize> {
    const Assoc2: usize;
}

impl<const N: usize> Trait2<N> for () {
    const Assoc2: usize = N - 1;
}


pub const fn foo2<const N: usize>() where (): Trait2<N> {
    let bar2 = [(); <()>::Assoc2];
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {
    foo::<0>();
    foo2::<0>();
}

