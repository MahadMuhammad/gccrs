fn add_one(x: i32) -> i32 {
    x + 1
}

fn add_two(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    add_one(2, 2); // { dg-error ".E0061." "" { target *-*-* } }
    add_one(no_such_local, 10); // { dg-error ".E0061." "" { target *-*-* } }
// { dg-error ".E0061." "" { target *-*-* } .-2 }
    add_one(10, no_such_local); // { dg-error ".E0061." "" { target *-*-* } }
// { dg-error ".E0061." "" { target *-*-* } .-2 }
    add_two(10, no_such_local, 10); // { dg-error ".E0061." "" { target *-*-* } }
// { dg-error ".E0061." "" { target *-*-* } .-2 }
    add_two(no_such_local, 10, 10); // { dg-error ".E0061." "" { target *-*-* } }
// { dg-error ".E0061." "" { target *-*-* } .-2 }
    add_two(10, 10, no_such_local); // { dg-error ".E0061." "" { target *-*-* } }
// { dg-error ".E0061." "" { target *-*-* } .-2 }
}

