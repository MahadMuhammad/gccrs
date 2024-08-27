struct Foo<T = impl Copy>(T);
// { dg-error ".E0562." "" { target *-*-* } .-1 }

type Result<T, E = impl std::error::Error> = std::result::Result<T, E>;
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// should not cause ICE
fn x() -> Foo {
    Foo(0)
}

fn main() -> Result<()> {}

