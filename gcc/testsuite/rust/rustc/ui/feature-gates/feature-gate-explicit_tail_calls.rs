pub fn you<T>() -> T {
    become bottom(); // { dg-error ".E0658." "" { target *-*-* } }
}

pub fn bottom<T>() -> T {
    become you(); // { dg-error ".E0658." "" { target *-*-* } }
}

fn main() {}

