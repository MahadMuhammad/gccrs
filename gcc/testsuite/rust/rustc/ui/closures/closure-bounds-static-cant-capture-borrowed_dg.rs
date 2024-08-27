fn bar<F>(blk: F) where F: FnOnce() + 'static {
}

fn foo(x: &()) {
    bar(|| {
// { dg-error ".E0373." "" { target *-*-* } .-1 }
// { dg-error ".E0373." "" { target *-*-* } .-2 }
        let _ = x;
    })
}

fn main() {
}

