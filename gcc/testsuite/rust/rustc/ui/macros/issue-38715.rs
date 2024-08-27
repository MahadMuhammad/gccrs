#[macro_export]
macro_rules! foo { () => {} }

#[macro_export]
macro_rules! foo { () => {} } // { dg-error ".E0428." "" { target *-*-* } }

mod inner1 {
    #[macro_export]
    macro_rules! bar { () => {} }
}

mod inner2 {
    #[macro_export]
    macro_rules! bar { () => {} } // { dg-error ".E0428." "" { target *-*-* } }
}

fn main() {}

