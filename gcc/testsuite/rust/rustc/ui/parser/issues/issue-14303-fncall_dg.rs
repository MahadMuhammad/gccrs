//@ revisions: full generic_arg
// can't run rustfix because it doesn't handle multipart suggestions correctly
// we need the above to avoid ast borrowck failure in recovered code
#![cfg_attr(generic_arg, feature(generic_arg_infer))]


struct S<'a, T> {
    a: &'a T,
    b: &'a T,
}

fn foo<'a, 'b>(start: &'a usize, end: &'a usize) {
    let _x = (*start..*end)
        .map(|x| S { a: start, b: end })
        .collect::<Vec<S<_, 'a>>>();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {}

