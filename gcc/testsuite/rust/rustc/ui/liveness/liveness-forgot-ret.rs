fn god_exists(a: isize) -> bool { return god_exists(a); }
// { dg-warning "" "" { target *-*-* } .-1 }

fn f(a: isize) -> isize { if god_exists(a) { return 5; }; }
// { dg-error ".E0308." "" { target *-*-* } .-1 }

fn main() { f(12); }

