macro_rules! falsy {
    () => { false };
}

fn foo() {
    if true {
    } else false {
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn foo2() {
    if true {
    } else falsy() {
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn foo3() {
    if true {
    } else falsy();
// { dg-error "" "" { target *-*-* } .-1 }
}

fn foo4() {
    if true {
    } else loop{}
// { dg-error "" "" { target *-*-* } .-1 }
    {}
}

fn foo5() {
    if true {
    } else falsy!() {
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn foo6() {
    if true {
    } else falsy!();
// { dg-error "" "" { target *-*-* } .-1 }
}

fn foo7() {
    if true {
    } else falsy! {} {
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn foo8() {
    if true {
    } else falsy! {};
// { dg-error "" "" { target *-*-* } .-1 }
}

fn falsy() -> bool {
    false
}

fn main() {}

