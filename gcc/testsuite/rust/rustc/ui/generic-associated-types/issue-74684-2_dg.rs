trait Fun {
    type F<'a>: ?Sized;

    fn identity<'a>(t: &'a Self::F<'a>) -> &'a Self::F<'a> { t }
}

impl <T> Fun for T {
    type F<'a> = i32;
}

fn bug<'a, T: ?Sized + Fun<F<'a> = [u8]>>(t: Box<T>) -> &'static T::F<'a> {
    let a = [0; 1];
    let x = T::identity(&a); // { dg-error ".E0597." "" { target *-*-* } }
    todo!()
}


fn main() {
    let x = 10;

    bug(Box::new(x));
// { dg-error ".E0271." "" { target *-*-* } .-1 }
}

