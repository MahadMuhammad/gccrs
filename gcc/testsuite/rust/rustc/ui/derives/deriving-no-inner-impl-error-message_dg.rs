struct NoCloneOrEq;

#[derive(PartialEq)]
struct E {
    x: NoCloneOrEq // { dg-error ".E0369." "" { target *-*-* } }
}
#[derive(Clone)]
struct C {
    x: NoCloneOrEq
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}


fn main() {}

