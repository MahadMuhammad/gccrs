fn foo<const X: usize, const Y: usize>() -> usize {
    0
}

fn main() {
    foo::<0>();
// { dg-error ".E0107." "" { target *-*-* } .-1 }

    foo::<0, 0, 0>();
// { dg-error ".E0107." "" { target *-*-* } .-1 }
}

