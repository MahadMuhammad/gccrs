//@ check-pass
// Ensure that trailing semicolons cause warnings by default

macro_rules! foo {
    () => {
        true; // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
    }
}

fn main() {
    let _val = match true {
        true => false,
        _ => foo!()
    };
}

