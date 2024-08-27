fn copy<R: Unpin, W>(_: R, _: W) {}

fn f<T>(r: T) {
    let w = ();
    copy(r, w);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

