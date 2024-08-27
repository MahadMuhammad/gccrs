// issue#123911
// issue#123912

macro_rules! m {
    ($p: path) => {
        #[$p]
        struct S;
    };
}

macro_rules! p {
    () => {};
}

m!(generic<p!()>);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

fn main() {}

