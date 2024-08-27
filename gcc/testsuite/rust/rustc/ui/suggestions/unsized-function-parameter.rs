//@ run-rustfix

#![allow(dead_code, unused_variables)]

fn foo1(bar: str) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { help ".E0277." "" { target *-*-* } .-2 }
// { help ".E0277." "" { target *-*-* } .-3 }
// { help ".E0277." "" { target *-*-* } .-4 }

fn foo2(_bar: str) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { help ".E0277." "" { target *-*-* } .-2 }
// { help ".E0277." "" { target *-*-* } .-3 }
// { help ".E0277." "" { target *-*-* } .-4 }

fn foo3(_: str) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { help ".E0277." "" { target *-*-* } .-2 }
// { help ".E0277." "" { target *-*-* } .-3 }
// { help ".E0277." "" { target *-*-* } .-4 }

fn main() {}

