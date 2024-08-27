mod foo {
    pub fn bar() -> i32 {
        1
    }
}

fn bar() -> i32 {
    2
}

fn main() {
    let stderr = 3;
    eprintln!({stderr});
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    eprintln!({1});
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    eprintln!({foo::bar()});
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    eprintln!({bar()});
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    eprintln!({1; 2});
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

