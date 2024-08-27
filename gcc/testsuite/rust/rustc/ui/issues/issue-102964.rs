use std::rc::Rc;
type Foo<'a, T> = &'a dyn Fn(&T);
type RcFoo<'a, T> = Rc<Foo<'a, T>>;

fn bar_function<T>(function: Foo<T>) -> RcFoo<T> {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let rc = Rc::new(function);
}

fn main() {}

