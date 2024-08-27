// { dg-additional-options "-frust-edition= 2024" }
//@ compile-flags: -Zunstable-options
//@ revisions: classic structural both
#![allow(incomplete_features)]
#![cfg_attr(any(classic, both), feature(ref_pat_eat_one_layer_2024))]
#![cfg_attr(any(structural, both), feature(ref_pat_eat_one_layer_2024_structural))]

pub fn main() {
    if let Some(&mut Some(&_)) = &Some(&Some(0)) {
// { dg-error "" "" { target *-*-* } .-1 }
    }
    if let Some(&Some(&mut _)) = &Some(&mut Some(0)) {
// { dg-error "" "" { target *-*-* } .-1 }
    }
    if let Some(&Some(x)) = &mut Some(&Some(0)) {
        let _: &mut u32 = x;
// { dg-error "" "" { target *-*-* } .-1 }
    }
    if let Some(&Some(&mut _)) = &mut Some(&Some(0)) {
// { dg-error "" "" { target *-*-* } .-1 }
    }
    if let Some(&Some(Some((&mut _)))) = &Some(Some(&mut Some(0))) {
// { dg-error "" "" { target *-*-* } .-1 }
    }
    if let Some(&mut Some(x)) = &Some(Some(0)) {
// { dg-error "" "" { target *-*-* } .-1 }
    }
    if let Some(&mut Some(x)) = &Some(Some(0)) {
// { dg-error "" "" { target *-*-* } .-1 }
    }

    let &mut _ = &&0;
// { dg-error "" "" { target *-*-* } .-1 }

    let &mut _ = &&&&&&&&&&&&&&&&&&&&&&&&&&&&0;
// { dg-error "" "" { target *-*-* } .-1 }

    if let Some(&mut Some(&_)) = &Some(&mut Some(0)) {
// { dg-error "" "" { target *-*-* } .-1 }
    }

    if let Some(Some(&mut x)) = &Some(Some(&mut 0)) {
// { dg-error "" "" { target *-*-* } .-1 }
    }

    let &mut _ = &&mut 0;
// { dg-error "" "" { target *-*-* } .-1 }

    let &mut _ = &&&&&&&&&&&&&&&&&&&&&&&&&&&&mut 0;
// { dg-error "" "" { target *-*-* } .-1 }

    let &mut &mut &mut &mut _ = &mut &&&&mut &&&mut &mut 0;
// { dg-error "" "" { target *-*-* } .-1 }

    if let Some(&mut _) = &mut Some(&0) {
// { dg-error "" "" { target *-*-* } .-1 }
    }

    struct Foo(u8);

    let Foo(mut a) = &Foo(0);
// { dg-error "" "" { target *-*-* } .-1 }
    a = &42;

    let Foo(mut a) = &mut Foo(0);
// { dg-error "" "" { target *-*-* } .-1 }
    a = &mut 42;

    fn generic<R: Ref>() -> R {
        R::meow()
    }

    trait Ref: Sized {
        fn meow() -> Self;
    }

    impl Ref for &'static mut [(); 0] {
        fn meow() -> Self {
            &mut []
        }
    }

    let &_ = generic(); // { dg-error "" "" { target *-*-* } }
}

