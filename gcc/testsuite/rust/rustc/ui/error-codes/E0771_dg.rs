#![feature(adt_const_params, unsized_const_params)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn function_with_str<'a, const STRING: &'a str>() {} // { dg-error ".E0770." "" { target *-*-* } }

fn main() {
    function_with_str::<"Hello, world!">()
}

