// Regression test for #123154

struct AA {
    pub data: [&usize]
// { dg-error ".E0106." "" { target *-*-* } .-1 }
}

impl AA {
    const fn new() -> Self { }
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

static ST: AA = AA::new();

fn main() {}

