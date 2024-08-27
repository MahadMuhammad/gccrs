struct Wrong {
    x: i32; // { dg-error "" "" { target *-*-* } }
    y: i32,
    z: i32,
    h: i32,
}

fn oops(w: &Wrong) {
    w.x;
}

fn foo(w: &Wrong) {
    w.y;
}

fn haha(w: &Wrong) {
    w.z;
}

struct WrongWithType {
    x: 1, // { dg-error "" "" { target *-*-* } }
    y: i32,
    z: i32,
    h: i32,
}

fn oops_type(w: &WrongWithType) {
    w.x;
}

fn foo_type(w: &WrongWithType) {
    w.y;
}

fn haha_type(w: &WrongWithType) {
    w.z;
}

fn main() {
    let v = Wrong { x: 1, y: 2, z: 3, h: 4 };
    let x = WrongWithType { x: 1, y: 2, z: 3, h: 4 };
}

