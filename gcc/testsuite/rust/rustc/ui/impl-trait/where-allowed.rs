//! A simple test for testing many permutations of allowedness of
//! impl Trait
#![feature(impl_trait_in_fn_trait_return)]
#![feature(custom_inner_attributes)]
#![rustfmt::skip]
use std::fmt::Debug;

// Allowed
fn in_parameters(_: impl Debug) { panic!() }

// Allowed
fn in_return() -> impl Debug { panic!() }

// Allowed
fn in_adt_in_parameters(_: Vec<impl Debug>) { panic!() }

// Disallowed
fn in_fn_parameter_in_parameters(_: fn(impl Debug)) { panic!() }
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
fn in_fn_return_in_parameters(_: fn() -> impl Debug) { panic!() }
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
fn in_fn_parameter_in_return() -> fn(impl Debug) { panic!() }
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
fn in_fn_return_in_return() -> fn() -> impl Debug { panic!() }
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
fn in_dyn_Fn_parameter_in_parameters(_: &dyn Fn(impl Debug)) { panic!() }
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
fn in_dyn_Fn_return_in_parameters(_: &dyn Fn() -> impl Debug) { panic!() }
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
fn in_dyn_Fn_parameter_in_return() -> &'static dyn Fn(impl Debug) { panic!() }
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Allowed
fn in_dyn_Fn_return_in_return() -> &'static dyn Fn() -> impl Debug { panic!() }
// { dg-error ".E0283." "" { target *-*-* } .-1 }

// Disallowed
fn in_impl_Fn_parameter_in_parameters(_: &impl Fn(impl Debug)) { panic!() }
// { dg-error ".E0562." "" { target *-*-* } .-1 }
// { dg-error ".E0562." "" { target *-*-* } .-2 }

// Disallowed
fn in_impl_Fn_return_in_parameters(_: &impl Fn() -> impl Debug) { panic!() }
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
fn in_impl_Fn_parameter_in_return() -> &'static impl Fn(impl Debug) { panic!() }
// { dg-error ".E0562." "" { target *-*-* } .-1 }
// { dg-error ".E0562." "" { target *-*-* } .-2 }

// Allowed
fn in_impl_Fn_return_in_return() -> &'static impl Fn() -> impl Debug { panic!() }
// { dg-error ".E0283." "" { target *-*-* } .-1 }

// Disallowed
fn in_Fn_parameter_in_generics<F: Fn(impl Debug)> (_: F) { panic!() }
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
fn in_Fn_return_in_generics<F: Fn() -> impl Debug> (_: F) { panic!() }
// { dg-error ".E0562." "" { target *-*-* } .-1 }


// Allowed
fn in_impl_Trait_in_parameters(_: impl Iterator<Item = impl Iterator>) { panic!() }

// Allowed
fn in_impl_Trait_in_return() -> impl IntoIterator<Item = impl IntoIterator> {
    vec![vec![0; 10], vec![12; 7], vec![8; 3]]
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

// Disallowed
struct InBraceStructField { x: impl Debug }
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
struct InAdtInBraceStructField { x: Vec<impl Debug> }
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
struct InTupleStructField(impl Debug);
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
enum InEnum {
    InBraceVariant { x: impl Debug },
// { dg-error ".E0562." "" { target *-*-* } .-1 }
    InTupleVariant(impl Debug),
// { dg-error ".E0562." "" { target *-*-* } .-1 }
}

// Allowed
trait InTraitDefnParameters {
    fn in_parameters(_: impl Debug);
}

// Allowed
trait InTraitDefnReturn {
    fn in_return() -> impl Debug;
}

// Allowed and disallowed in trait impls
trait DummyTrait {
    type Out;
    fn in_trait_impl_parameter(_: impl Debug);
    fn in_trait_impl_return() -> Self::Out;
}
impl DummyTrait for () {
    type Out = impl Debug;
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }

    fn in_trait_impl_parameter(_: impl Debug) { }
    // Allowed

    fn in_trait_impl_return() -> impl Debug { () }
// { dg-error ".E0053." "" { target *-*-* } .-1 }
    // Allowed
}

// Allowed
struct DummyType;
impl DummyType {
    fn in_inherent_impl_parameters(_: impl Debug) { }
    fn in_inherent_impl_return() -> impl Debug { () }
}

// Disallowed
extern "C" {
    fn in_foreign_parameters(_: impl Debug);
// { dg-error ".E0562." "" { target *-*-* } .-1 }

    fn in_foreign_return() -> impl Debug;
// { dg-error ".E0562." "" { target *-*-* } .-1 }
}

// Allowed
extern "C" fn in_extern_fn_parameters(_: impl Debug) {
}

// Allowed
extern "C" fn in_extern_fn_return() -> impl Debug {
    22
}

type InTypeAlias<R> = impl Debug;
// { dg-error ".E0658." "" { target *-*-* } .-1 }

type InReturnInTypeAlias<R> = fn() -> impl Debug;
// { dg-error ".E0562." "" { target *-*-* } .-1 }
// { dg-error ".E0562." "" { target *-*-* } .-2 }

// Disallowed in impl headers
impl PartialEq<impl Debug> for () {
// { dg-error ".E0562." "" { target *-*-* } .-1 }
}

// Disallowed in impl headers
impl PartialEq<()> for impl Debug {
// { dg-error ".E0562." "" { target *-*-* } .-1 }
}

// Disallowed in inherent impls
impl impl Debug {
// { dg-error ".E0562." "" { target *-*-* } .-1 }
}

// Disallowed in inherent impls
struct InInherentImplAdt<T> { t: T }
impl InInherentImplAdt<impl Debug> {
// { dg-error ".E0562." "" { target *-*-* } .-1 }
}

// Disallowed in where clauses
fn in_fn_where_clause()
    where impl Debug: Debug
// { dg-error ".E0562." "" { target *-*-* } .-1 }
{
}

// Disallowed in where clauses
fn in_adt_in_fn_where_clause()
    where Vec<impl Debug>: Debug
// { dg-error ".E0562." "" { target *-*-* } .-1 }
{
}

// Disallowed
fn in_trait_parameter_in_fn_where_clause<T>()
    where T: PartialEq<impl Debug>
// { dg-error ".E0562." "" { target *-*-* } .-1 }
{
}

// Disallowed
fn in_Fn_parameter_in_fn_where_clause<T>()
    where T: Fn(impl Debug)
// { dg-error ".E0562." "" { target *-*-* } .-1 }
{
}

// Disallowed
fn in_Fn_return_in_fn_where_clause<T>()
    where T: Fn() -> impl Debug
// { dg-error ".E0562." "" { target *-*-* } .-1 }
{
}

// Disallowed
struct InStructGenericParamDefault<T = impl Debug>(T);
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
enum InEnumGenericParamDefault<T = impl Debug> { Variant(T) }
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
trait InTraitGenericParamDefault<T = impl Debug> {}
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
type InTypeAliasGenericParamDefault<T = impl Debug> = T;
// { dg-error ".E0562." "" { target *-*-* } .-1 }

// Disallowed
impl <T = impl Debug> T {}
// { dg-error ".E0118." "" { target *-*-* } .-1 }
// { dg-warning ".E0118." "" { target *-*-* } .-2 }
// { dg-error ".E0118." "" { target *-*-* } .-3 }
// { dg-error ".E0118." "" { target *-*-* } .-4 }

// Disallowed
fn in_method_generic_param_default<T = impl Debug>(_: T) {}
// { dg-error ".E0562." "" { target *-*-* } .-1 }
// { dg-warning ".E0562." "" { target *-*-* } .-2 }
// { dg-error ".E0562." "" { target *-*-* } .-3 }

fn main() {
    let _in_local_variable: impl Fn() = || {};
// { dg-error ".E0562." "" { target *-*-* } .-1 }
    let _in_return_in_local_variable = || -> impl Fn() { || {} };
// { dg-error ".E0562." "" { target *-*-* } .-1 }
}

