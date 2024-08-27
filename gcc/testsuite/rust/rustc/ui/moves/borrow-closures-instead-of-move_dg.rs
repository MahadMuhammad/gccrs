fn takes_fn(f: impl Fn()) { // { help "" "" { target *-*-* } }
    loop {
        takes_fnonce(f);
// { dg-error ".E0382." "" { target *-*-* } .-1 }
// { help ".E0382." "" { target *-*-* } .-2 }
    }
}

fn takes_fn_mut(m: impl FnMut()) { // { help "" "" { target *-*-* } }
    if maybe() {
        takes_fnonce(m);
// { help "" "" { target *-*-* } .-1 }
    }
    takes_fnonce(m);
// { dg-error ".E0382." "" { target *-*-* } .-1 }
}

fn has_closure() {
    let mut x = 0;
    let mut closure = || {
        x += 1;
    };
    takes_fnonce(closure);
// { help "" "" { target *-*-* } .-1 }
    closure();
// { dg-error ".E0382." "" { target *-*-* } .-1 }
}

fn maybe() -> bool {
    false
}

// Could also be Fn[Mut], here it doesn't matter
fn takes_fnonce(_: impl FnOnce()) {}

fn main() {}

