// Also includes more Self usages per #93796

fn foo(_: Self) {
// { dg-error ".E0411." "" { target *-*-* } .-1 }
}

fn foo2() {
    let x: Self;
// { dg-error ".E0411." "" { target *-*-* } .-1 }
}

type Foo<T>
where
    Self: Clone,
// { dg-error ".E0411." "" { target *-*-* } .-1 }
= Vec<T>;

const FOO: Self = 0;
// { dg-error ".E0411." "" { target *-*-* } .-1 }

const FOO2: u32 = Self::bar();
// { dg-error ".E0433." "" { target *-*-* } .-1 }

static FOO_S: Self = 0;
// { dg-error ".E0411." "" { target *-*-* } .-1 }

static FOO_S2: u32 = Self::bar();
// { dg-error ".E0433." "" { target *-*-* } .-1 }

fn main() {}

