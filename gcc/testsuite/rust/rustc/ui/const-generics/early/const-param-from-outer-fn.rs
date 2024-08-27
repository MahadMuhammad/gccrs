fn foo<const X: u32>() {
    fn bar() -> u32 {
        X // { dg-error ".E0401." "" { target *-*-* } }
    }
}

fn main() {}

