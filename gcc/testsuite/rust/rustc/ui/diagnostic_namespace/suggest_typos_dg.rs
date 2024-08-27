#![deny(unknown_or_malformed_diagnostic_attributes)]

#[diagnostic::onunimplemented]
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
trait X{}

#[diagnostic::un_onimplemented]
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
trait Y{}

#[diagnostic::on_implemented]
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
trait Z{}

fn main(){}

