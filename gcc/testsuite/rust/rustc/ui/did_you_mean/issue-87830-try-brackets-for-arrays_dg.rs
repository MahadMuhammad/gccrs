fn main() {}

const FOO: [u8; 3] = {
// { dg-error "" "" { target *-*-* } .-1 }
    1, 2, 3
};

const BAR: [&str; 3] = {"one", "two", "three"};
// { dg-error "" "" { target *-*-* } .-1 }

fn foo() {
    {1, 2, 3};
// { dg-error "" "" { target *-*-* } .-1 }
}

fn bar() {
    1, 2, 3 // { dg-error "" "" { target *-*-* } }
}

