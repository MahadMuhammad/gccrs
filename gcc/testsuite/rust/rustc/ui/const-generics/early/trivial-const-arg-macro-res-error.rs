// This is a regression test for #128016.

macro_rules! len {
    () => {
        target
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    };
}

fn main() {
    let val: [str; len!()] = [];
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

