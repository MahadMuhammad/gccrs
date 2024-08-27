struct X<F> where F: FnOnce() + 'static + Send {
    field: F,
}

fn foo<F>(blk: F) -> X<F> where F: FnOnce() + 'static {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    return X { field: blk };
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {
}

