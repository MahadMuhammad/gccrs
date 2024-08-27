//@ compile-flags: -Znext-solver=coherence

trait Mirror {
    type Assoc;
}
impl<T> Mirror for T {
    type Assoc = T;
}

trait Foo {}

// Even though using fulfillment in coherence allows us to figure out that
// `?T = ()`, we still treat it as incoherent because `(): Iterator` may be
// added upstream.
impl<T> Foo for T where (): Mirror<Assoc = T> {}
// { dg-note "" "" { target *-*-* } .-1 }
impl<T> Foo for T where T: Iterator {}
// { dg-error ".E0119." "" { target *-*-* } .-1 }
// { dg-note ".E0119." "" { target *-*-* } .-2 }
// { dg-note ".E0119." "" { target *-*-* } .-3 }

fn main() {}

