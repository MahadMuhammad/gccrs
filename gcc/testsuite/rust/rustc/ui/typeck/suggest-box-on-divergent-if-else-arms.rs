//@ run-rustfix
trait Trait {}
struct Struct;
impl Trait for Struct {}
fn foo() -> Box<dyn Trait> {
    Box::new(Struct)
}
fn bar() -> impl Trait {
    Struct
}
fn main() {
    let _ = if true {
        Struct
    } else {
        foo() // { dg-error ".E0308." "" { target *-*-* } }
    };
    let _ = if true {
        foo()
    } else {
        Struct // { dg-error ".E0308." "" { target *-*-* } }
    };
    let _ = if true {
        Struct
    } else {
        bar() // { dg-error ".E0308." "" { target *-*-* } }
    };
    let _ = if true {
        bar()
    } else {
        Struct // { dg-error ".E0308." "" { target *-*-* } }
    };
}

