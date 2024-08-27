// { dg-additional-options "-frust-edition=2021" }

#![feature(rustc_attrs)]

// Test that we can't move out of struct that impls `Drop`.


use std::rc::Rc;

// Test that we restrict precision when moving not-`Copy` types, if any of the parent paths
// implement `Drop`. This is to ensure that we don't move out of a type that implements Drop.
pub fn test1() {
    struct Foo(Rc<i32>);

    impl Drop for Foo {
        fn drop(self: &mut Foo) {}
    }

    let f = Foo(Rc::new(1));
    let x = #[rustc_capture_analysis] move || {
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
// { dg-error ".E0658." "" { target *-*-* } .-4 }
// { dg-error ".E0658." "" { target *-*-* } .-5 }
        println!("{:?}", f.0);
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };

    x();
}

// Test that we don't restrict precision when moving `Copy` types(i.e. when copying),
// even if any of the parent paths implement `Drop`.
fn test2() {
    struct Character {
        hp: u32,
        name: String,
    }

    impl Drop for Character {
        fn drop(&mut self) {}
    }

    let character = Character { hp: 100, name: format!("A") };

    let c = #[rustc_capture_analysis] move || {
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
// { dg-error ".E0658." "" { target *-*-* } .-4 }
// { dg-error ".E0658." "" { target *-*-* } .-5 }
        println!("{}", character.hp)
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };

    c();

    println!("{}", character.name);
}

fn main() {}

