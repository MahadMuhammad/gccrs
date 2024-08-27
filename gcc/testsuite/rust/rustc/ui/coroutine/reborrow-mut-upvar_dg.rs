//@ run-pass

#![feature(coroutines)]

fn _run(bar: &mut i32) {
    #[coroutine] || { // { dg-warning "" "" { target *-*-* } }
        {
            let _baz = &*bar;
            yield;
        }

        *bar = 2;
    };
}

fn main() {}

