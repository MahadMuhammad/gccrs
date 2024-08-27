// { dg-additional-options "-frust-edition=2021" }
macro_rules! a {
    ( ) => {
        impl<'b> c for d {
            e::<f'g> // { dg-error "" "" { target *-*-* } }
        }
    };
}
fn main() {}

