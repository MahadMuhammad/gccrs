// Regression test for #122561

fn for_infinite() -> bool {
    for i in 0.. {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        return false;
    }
}

fn for_finite() -> String {
    for i in 0..5 {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        return String::from("test");
    }
}

fn for_zero_times() -> bool {
    for i in 0..0 {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        return true;
    }
}

fn for_never_type() -> ! {
    for i in 0..5 {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
}

// Entire function on a single line.
// Tests that we format the suggestion
// correctly in this case
fn for_single_line() -> bool { for i in 0.. { return false; } }
// { dg-error ".E0308." "" { target *-*-* } .-1 }

// Loop in an anon const in function args
// Tests that we:
// a. deal properly with this complex case
// b. format the suggestion correctly so
//    that it's readable
fn for_in_arg(a: &[(); for x in 0..2 {}]) -> bool {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    true
}

fn while_inifinite() -> bool {
    while true {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-warning ".E0308." "" { target *-*-* } .-2 }
        return true;
    }
}

fn while_finite() -> bool {
    let mut i = 0;
    while i < 3 {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        i += 1;
        return true;
    }
}

fn while_zero_times() -> bool {
    while false {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        return true;
    }
}

fn while_never_type() -> ! {
    while true {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-warning ".E0308." "" { target *-*-* } .-2 }
    }
}

// No type mismatch error in this case
fn loop_() -> bool {
    loop {
        return true;
    }
}

const C: i32 = {
    for i in 0.. {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    }
};

fn main() {
    let _ = [10; {
        for i in 0..5 {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
        }
    }];

    let _ = [10; {
        while false {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        }
    }];


    let _ = |a: &[(); for x in 0..2 {}]| {};
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
}

