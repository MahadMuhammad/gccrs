#![feature(never_type)]

fn main() {
    let val: ! = loop { break break; };
// { dg-error ".E0308." "" { target *-*-* } .-1 }

    loop {
        if true {
            break "asdf";
        } else {
            break 123; // { dg-error ".E0308." "" { target *-*-* } }
        }
    };

    let _: i32 = loop {
        break "asdf"; // { dg-error ".E0308." "" { target *-*-* } }
    };

    let _: i32 = 'outer_loop: loop {
        loop {
            break 'outer_loop "nope"; // { dg-error ".E0308." "" { target *-*-* } }
            break "ok";
        };
    };

    let _: Option<String> = loop {
        break; // { dg-error ".E0308." "" { target *-*-* } }
    };

    'while_loop: while true { // { dg-warning "" "" { target *-*-* } }
        break;
        break (); // { dg-error ".E0571." "" { target *-*-* } }
        loop {
            break 'while_loop 123;
// { dg-error ".E0571." "" { target *-*-* } .-1 }
            break 456;
            break 789;
        };
    }

    while let Some(_) = Some(()) {
        if break () { // { dg-error ".E0571." "" { target *-*-* } }
        }
    }

    while let Some(_) = Some(()) {
        break None;
// { dg-error ".E0571." "" { target *-*-* } .-1 }
    }

    'while_let_loop: while let Some(_) = Some(()) {
        loop {
            break 'while_let_loop "nope";
// { dg-error ".E0571." "" { target *-*-* } .-1 }
            break 33;
        };
    }

    for _ in &[1,2,3] {
        break (); // { dg-error ".E0571." "" { target *-*-* } }
        break [()];
// { dg-error ".E0571." "" { target *-*-* } .-1 }
    }

    'for_loop: for _ in &[1,2,3] {
        loop {
            break Some(3);
            break 'for_loop Some(17);
// { dg-error ".E0571." "" { target *-*-* } .-1 }
        };
    }

    let _: i32 = 'a: loop {
        let _: () = 'b: loop {
            break ('c: loop {
                break;
                break 'c 123; // { dg-error ".E0308." "" { target *-*-* } }
            });
            break 'a 123;
        };
    };

    loop {
        break (break, break); // { dg-error ".E0308." "" { target *-*-* } }
    };

    loop {
        break;
        break 2; // { dg-error ".E0308." "" { target *-*-* } }
    };

    loop {
        break 2;
        break; // { dg-error ".E0308." "" { target *-*-* } }
        break 4;
    };

    'LOOP: for _ in 0 .. 9 {
        break LOOP;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    }

    let _ = 'a: loop {
        loop {
            break; // This doesn't affect the expected break type of the 'a loop
            loop {
                loop {
                    break 'a 1;
                }
            }
        }
        break; // { dg-error ".E0308." "" { target *-*-* } }
    };

    let _ = 'a: loop {
        loop {
            break; // This doesn't affect the expected break type of the 'a loop
            loop {
                loop {
                    break 'a 1;
                }
            }
        }
        break 'a; // { dg-error ".E0308." "" { target *-*-* } }
    };

    loop {
        break;
        let _ = loop {
            break 2;
            loop {
                break;
            }
        };
        break 2; // { dg-error ".E0308." "" { target *-*-* } }
    }

    'a: loop {
        break;
        let _ = 'a: loop {
// { dg-warning "" "" { target *-*-* } .-1 }
            break 2;
            loop {
                break 'a; // { dg-error ".E0308." "" { target *-*-* } }
            }
        };
        break 2; // { dg-error ".E0308." "" { target *-*-* } }
    }

    'a: loop {
        break;
        let _ = 'a: loop {
// { dg-warning "" "" { target *-*-* } .-1 }
            break 'a 2;
            loop {
                break 'a; // { dg-error ".E0308." "" { target *-*-* } }
            }
        };
        break 2; // { dg-error ".E0308." "" { target *-*-* } }
    };

    loop { // point at the return type
        break 2; // { dg-error ".E0308." "" { target *-*-* } }
    }
}

