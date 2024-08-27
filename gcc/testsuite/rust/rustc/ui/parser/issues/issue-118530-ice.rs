fn bar() -> String {
    #[cfg]
    [1, 2, 3].iter() // { dg-error "" "" { target *-*-* } }
    #[feature]
    attr::fn bar() -> String { // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
    #[attr]
    [1, 2, 3].iter().map().collect::<String>()
    #[attr]

}()
}

fn main() { }

