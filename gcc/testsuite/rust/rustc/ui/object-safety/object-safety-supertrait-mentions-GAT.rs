// { dg-error "" "" { target *-*-* } }

trait GatTrait {
    type Gat<'a>
    where
        Self: 'a;
}

trait SuperTrait<T>: for<'a> GatTrait<Gat<'a> = T> {
    fn c(&self) -> dyn SuperTrait<T>;
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
}

fn main() {}

