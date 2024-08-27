struct Wrapper<T>(T);

fn foo<T>(foo: Wrapper<T>)
// { dg-error ".E0277." "" { target *-*-* } .-1 }
where
    T
    :
    ?
    Sized
{
    //
}

fn bar<T>(foo: Wrapper<T>)
// { dg-error ".E0277." "" { target *-*-* } .-1 }
where T: ?Sized
{
    //
}

fn qux<T>(foo: Wrapper<T>)
// { dg-error ".E0277." "" { target *-*-* } .-1 }
where
    T: ?Sized
{
    //
}


fn main() {}

