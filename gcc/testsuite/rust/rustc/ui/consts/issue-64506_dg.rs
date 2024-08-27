//@ check-fail

#[derive(Copy, Clone)]
pub struct ChildStdin {
    inner: AnonPipe,
}

#[derive(Copy, Clone)]
enum AnonPipe {}

const FOO: () = {
    union Foo {
        a: ChildStdin,
        b: (),
    }
    let x = unsafe { Foo { b: () }.a };
// { dg-error ".E0080." "" { target *-*-* } .-1 }
    let x = &x.inner;
};

fn main() {}

