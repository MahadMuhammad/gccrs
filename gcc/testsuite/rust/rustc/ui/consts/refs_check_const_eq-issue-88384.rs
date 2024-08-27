#![feature(fn_traits)]
#![feature(adt_const_params, unsized_const_params)]
// { dg-warning "" "" { target *-*-* } .-1 }

#[derive(PartialEq, Eq)]
struct CompileTimeSettings {
    hooks: &'static [fn()],
}

struct Foo<const T: CompileTimeSettings>;
// { dg-error ".E0741." "" { target *-*-* } .-1 }

impl<const T: CompileTimeSettings> Foo<T> {
// { dg-error ".E0741." "" { target *-*-* } .-1 }
    fn call_hooks() {}
}

fn main() {
    const SETTINGS: CompileTimeSettings = CompileTimeSettings { hooks: &[] };

    Foo::<SETTINGS>::call_hooks();
}

