fn main() {
    let x = 1;
    let y = 2;
    let value = 3;

    match value {
        Some(x) if x == y {
            self.next_token()?; // { dg-error "" "" { target *-*-* } }
        },
        _ => {}
    }
    let _: i32 = (); // { dg-error ".E0308." "" { target *-*-* } }
}

struct Foo {
    value: usize
}

fn foo(a: Option<&mut Foo>, b: usize) {
    match a {
        Some(a) if a.value == b {
            a.value = 1; // { dg-error "" "" { target *-*-* } }
        },
        _ => {}
    }
    let _: i32 = (); // { dg-error ".E0308." "" { target *-*-* } }
}

fn bar(a: Option<&mut Foo>, b: usize) {
    match a {
        Some(a) if a.value == b {
            a.value, // { dg-error "" "" { target *-*-* } }
        } => {
        }
        _ => {}
    }
    let _: i32 = (); // { dg-error ".E0308." "" { target *-*-* } }
}

