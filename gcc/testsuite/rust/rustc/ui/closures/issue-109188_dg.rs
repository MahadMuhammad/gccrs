enum Either {
    One(X),
    Two(X),
}

struct X(Y);

struct Y;

fn consume_fnmut(f: &mut dyn FnMut()) {
    f();
}

fn move_into_fnmut() {
    let x = move_into_fnmut();
    consume_fnmut(&mut || {
        let Either::One(_t) = x; // { dg-error ".E0308." "" { target *-*-* } }
        let Either::Two(_t) = x; // { dg-error ".E0308." "" { target *-*-* } }
    });
}

fn main() { }

