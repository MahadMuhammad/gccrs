#![feature(unboxed_closures)]

trait SomeTrait<'a> {
    type Associated;
}

fn give_me_ice<T>() {
    callee::<fn(&()) -> <T as SomeTrait<'_>>::Associated>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
}

fn callee<T: Fn<(&'static (),)>>() {
    println!("{}", std::any::type_name::<<T as FnOnce<(&'static (),)>>::Output>());
}

fn main() {
    give_me_ice::<()>();
}

