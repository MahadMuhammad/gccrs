//@ normalize-stderr-test: "long-type-\d+" -> "long-type-hash"

fn id(
    f: &dyn Fn(u32),
) -> &dyn Fn(
    &dyn Fn(
        &dyn Fn(
            &dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(&dyn Fn(u32))))))))),
        ),
    ),
) {
    f
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

