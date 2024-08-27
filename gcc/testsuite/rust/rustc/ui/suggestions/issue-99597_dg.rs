#![allow(dead_code)]

trait T1 { }

trait T2 {
    fn test(&self) { }
}

fn go(s: &impl T1) {
// { suggestion "" "" { target *-*-* } .-1 }
    s.test();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

fn main() { }

