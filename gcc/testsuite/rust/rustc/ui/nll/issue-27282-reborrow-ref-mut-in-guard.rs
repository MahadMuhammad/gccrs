// Issue 27282: This is a variation on issue-27282-move-ref-mut-into-guard.rs
//
// It reborrows instead of moving the `ref mut` pattern borrow. This
// means that our conservative check for mutation in guards will
// reject it. But I want to make sure that we continue to reject it
// (under NLL) even when that conservative check goes away.

#![feature(if_let_guard)]

fn main() {
    let mut b = &mut true;
    match b {
        &mut false => {},
        ref mut r if { (|| { let bar = &mut *r; **bar = false; })();
// { dg-error ".E0596." "" { target *-*-* } .-1 }
                             false } => { &mut *r; },
        &mut true => { println!("You might think we should get here"); },
        _ => panic!("surely we could never get here, since rustc warns it is unreachable."),
    }

    let mut b = &mut true;
    match b {
        &mut false => {},
        ref mut r if let Some(()) = { (|| { let bar = &mut *r; **bar = false; })();
// { dg-error ".E0596." "" { target *-*-* } .-1 }
                             None } => { &mut *r; },
        &mut true => {},
        _ => {},
    }
}

