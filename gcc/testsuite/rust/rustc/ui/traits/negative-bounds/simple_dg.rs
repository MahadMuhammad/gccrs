#![feature(negative_bounds, negative_impls)]

fn not_copy<T: !Copy>() {}

fn neg_param_env<T: !Copy>() {
    not_copy::<T>();
}

fn pos_param_env<T: Copy>() {
    not_copy::<T>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn unknown<T>() {
    not_copy::<T>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

struct NotCopyable;
impl !Copy for NotCopyable {}

fn neg_impl() {
    not_copy::<NotCopyable>();
}

#[derive(Copy, Clone)]
struct Copyable;

fn pos_impl() {
    not_copy::<Copyable>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

struct NotNecessarilyCopyable;

fn unknown_impl() {
    not_copy::<NotNecessarilyCopyable>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

