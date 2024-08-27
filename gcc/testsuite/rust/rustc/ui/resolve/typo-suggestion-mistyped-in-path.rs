struct Struct;
// { dg-note "" "" { target *-*-* } .-1 }

impl Struct {
    fn foo() { }
}

mod module {
    fn foo() { }

    struct Struct;

    impl Struct {
        fn foo() { }
    }
}

trait Trait {
    fn foo();
}

fn main() {
    Struct::fob();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-note ".E0599." "" { target *-*-* } .-2 }

    Struc::foo();
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-note ".E0433." "" { target *-*-* } .-2 }

    modul::foo();
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-note ".E0433." "" { target *-*-* } .-2 }

    module::Struc::foo();
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-note ".E0433." "" { target *-*-* } .-2 }

    Trai::foo();
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-note ".E0433." "" { target *-*-* } .-2 }
}

