#![feature(coroutines, stmt_expr_attributes)]

fn main() {
    let _coroutine = #[coroutine]
    || {
        yield ((), ((), ()));
        yield ((), ());
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    };
}

