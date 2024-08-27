//! Show an uninformative diagnostic that we could possibly improve in the future

trait Trait<T> {}

impl<T, U> Trait<T> for U {}

fn hello() -> &'static (dyn Trait<impl Sized> + Send) {
// { dg-error ".E0282." "" { target *-*-* } .-1 }
    if false {
        let x = hello();
        let _: &'static dyn Trait<()> = &x;
        //^ Note the extra `&`, paired with the blanket impl causing
        // `impl Sized` to never get a hidden type registered.
    }
    todo!()
}

fn main() {}

