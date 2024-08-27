fn main() {
    let a: i8 = loop {
        1 // { dg-error ".E0308." "" { target *-*-* } }
    };

    let b: i8 = loop {
        break 1;
    };
}

fn foo() -> i8 {
    let a: i8 = loop {
        1 // { dg-error ".E0308." "" { target *-*-* } }
    };

    let b: i8 = loop {
        break 1;
    };

    loop {
        1 // { dg-error ".E0308." "" { target *-*-* } }
    }

    loop {
        return 1;
    }

    loop {
        1 // { dg-error ".E0308." "" { target *-*-* } }
    }
}

