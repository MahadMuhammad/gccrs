#![feature(const_trait_impl, effects)] // { dg-warning "" "" { target *-*-* } }

const fn foo() {}

#[const_trait]
trait Bar {
    fn bar();
}

impl Bar for () {
    fn bar() {}
}

fn main() {
    foo::<true>();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
    <() as Bar<true>>::bar();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
}

const FOO: () = {
    foo::<false>();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
    <() as Bar<false>>::bar();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
};

