fn borrowck_catch() {
    let foo = String::new();
    let _bar = foo;
    let _baz = [foo; 0]; // { dg-error ".E0382." "" { target *-*-* } }
}

const _: [String; 0] = [String::new(); 0];
// { dg-error ".E0493." "" { target *-*-* } .-1 }

fn must_be_init() {
    let x: u8;
    let _ = [x; 0]; // { dg-error ".E0381." "" { target *-*-* } }
}

fn main() {}

