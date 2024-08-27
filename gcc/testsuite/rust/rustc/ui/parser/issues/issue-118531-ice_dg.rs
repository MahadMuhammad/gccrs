fn bar() -> String {
    #[cfg(feature = )]
    [1, 2, 3].iter().map().collect::<String>() // { dg-error "" "" { target *-*-* } }

    #[attr] // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    String::new()
}

fn main() { }

