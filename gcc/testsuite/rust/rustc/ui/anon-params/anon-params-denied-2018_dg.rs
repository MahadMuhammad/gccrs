// Tests that anonymous parameters are a hard error in edition 2018.

// { dg-additional-options "-frust-edition=2018" }

trait T {
    fn foo(i32); // { dg-error "" "" { target *-*-* } }

    // Also checks with `&`
    fn foo_with_ref(&mut i32);
// { dg-error "" "" { target *-*-* } .-1 }

    fn foo_with_qualified_path(<Bar as T>::Baz);
// { dg-error "" "" { target *-*-* } .-1 }

    fn foo_with_qualified_path_and_ref(&<Bar as T>::Baz);
// { dg-error "" "" { target *-*-* } .-1 }

    fn foo_with_multiple_qualified_paths(<Bar as T>::Baz, <Bar as T>::Baz);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    fn bar_with_default_impl(String, String) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    // do not complain about missing `b`
    fn baz(a:usize, b, c: usize) -> usize { // { dg-error "" "" { target *-*-* } }
        a + b + c
    }
}

fn main() {}

