// { dg-additional-options "-frust-edition= 2018" }

// https://github.com/rust-lang/rust/issues/98291

macro_rules! wrap {
    () => {
        macro_rules! _a {
            () => {
                ""
            };
        }
    };
}

wrap!();

fn main() {
    format_args!(a!()); // { dg-error "" "" { target *-*-* } }
    env!(a!()); // { dg-error "" "" { target *-*-* } }
}

