//@ run-rustfix
fn foo<T: Default>(list: &mut Vec<T>) {
    let mut cloned_items = Vec::new();
    for v in list.iter() {
        cloned_items.push(v.clone())
    }
    list.push(T::default());
// { dg-error ".E0502." "" { target *-*-* } .-1 }
    drop(cloned_items);
}
fn bar<T: std::fmt::Display>(x: T) {
    let a = &x;
    let b = a.clone();
    drop(x);
// { dg-error ".E0505." "" { target *-*-* } .-1 }
    println!("{b}");
}
#[derive(Debug)]
struct A;
fn qux(x: A) {
    let a = &x;
    let b = a.clone();
    drop(x);
// { dg-error ".E0505." "" { target *-*-* } .-1 }
    println!("{b:?}");
}
fn main() {
    foo(&mut vec![1, 2, 3]);
    bar("");
    qux(A);
}

