struct MyError;

fn foo(x: bool) -> Result<(), MyError> {
    if x {
        Err(MyError);
// { dg-error ".E0282." "" { target *-*-* } .-1 }
    }

    Ok(())
}

fn bar(x: bool) -> Result<(), MyError> {
    if x {
        Ok(());
// { dg-error ".E0282." "" { target *-*-* } .-1 }
    }

    Ok(())
}

fn baz(x: bool) -> Result<(), MyError> {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    if x {
        1;
    }

    Err(MyError);
}

fn error() -> Result<(), MyError> {
    Err(MyError)
}

fn bak(x: bool) -> Result<(), MyError> {
    if x {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        error();
    } else {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        error();
    }
}

fn bad(x: bool) -> Result<(), MyError> {
    Err(MyError); // { dg-error ".E0282." "" { target *-*-* } }
    Ok(())
}

fn with_closure<F, A, B>(_: F) -> i32
where
    F: FnOnce(A, B),
{
    0
}

fn a() -> i32 {
    with_closure(|x: u32, y| {}); // { dg-error ".E0282." "" { target *-*-* } }
    0
}

fn main() {}

