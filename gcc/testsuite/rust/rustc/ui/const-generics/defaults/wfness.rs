struct Ooopsies<const N: u8 = { u8::MAX + 1 }>;
// { dg-error ".E0080." "" { target *-*-* } .-1 }

trait Trait<const N: u8> {}
impl Trait<3> for () {}
struct WhereClause<const N: u8 = 2>
where
    (): Trait<N>;
// { dg-error ".E0277." "" { target *-*-* } .-1 }

trait Traitor<T, const N: u8> {}
struct WhereClauseTooGeneric<T = u32, const N: u8 = 2>(T)
where
    (): Traitor<T, N>;

// no error on struct def
struct DependentDefaultWfness<const N: u8 = 1, T = WhereClause<N>>(T);
fn foo() -> DependentDefaultWfness {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    loop {}
}

fn main() {}

