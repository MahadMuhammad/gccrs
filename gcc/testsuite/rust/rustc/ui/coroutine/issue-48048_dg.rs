#![feature(coroutines)]

fn main() {
    let x = (|_| {},);

    #[coroutine] || {
        let x = x;

        x.0({ // { dg-error ".E0626." "" { target *-*-* } }
            yield;
        });
    };
}

