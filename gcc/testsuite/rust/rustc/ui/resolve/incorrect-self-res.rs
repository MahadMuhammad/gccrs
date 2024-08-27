fn module() {
    fn test(&mut self) {
// { dg-error "" "" { target *-*-* } .-1 }
    }
    mod Self {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn trait_() {
    fn test(&mut self) {
// { dg-error "" "" { target *-*-* } .-1 }
    }
    trait Self {}
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

