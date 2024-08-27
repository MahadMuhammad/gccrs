trait Trait {}
struct Struct;
impl Trait for Struct {}
fn foo() -> impl Trait {
    Struct
}
fn main() {
    let a: Box<dyn Trait> = if true {
        Box::new(Struct)
    } else {
        foo() // { dg-error ".E0308." "" { target *-*-* } }
    };
    let a: dyn Trait = if true {
        Struct // { dg-error ".E0308." "" { target *-*-* } }
    } else {
        foo() // { dg-error ".E0308." "" { target *-*-* } }
    };
}

