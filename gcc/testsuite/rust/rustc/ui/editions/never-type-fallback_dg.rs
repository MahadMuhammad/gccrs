//@ revisions: e2021 e2024
//
// { dg-additional-options "-frust-edition= 2021" }
// { dg-additional-options "-frust-edition= 2024" }
//@[e2024] compile-flags: -Zunstable-options
//
//@ run-pass
//@ check-run-results

fn main() {
    print_return_type_of(|| panic!());
}

fn print_return_type_of<R>(_: impl FnOnce() -> R) {
    println!("return type = {}", std::any::type_name::<R>());
}

