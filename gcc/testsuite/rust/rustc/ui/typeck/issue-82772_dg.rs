// { dg-additional-options "-frust-edition=2018" }

fn main() {
    use a::ModPrivateStruct;
    let Box { 0: _, .. }: Box<()>; // { dg-error ".E0451." "" { target *-*-* } }
    let Box { 1: _, .. }: Box<()>; // { dg-error ".E0451." "" { target *-*-* } }
    let ModPrivateStruct { 1: _, .. } = ModPrivateStruct::default(); // { dg-error ".E0451." "" { target *-*-* } }
}

mod a {
    #[derive(Default)]
    pub struct ModPrivateStruct(u8, u8);
}

