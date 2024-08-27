#![feature(link_cfg)]

// Top-level ill-formed
#[link(name = "...", "literal")] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", unknown)] // { dg-error "" "" { target *-*-* } }
extern "C" {}

// Duplicate arguments
#[link(name = "foo", name = "bar")] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", kind = "dylib", kind = "bar")] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", modifiers = "+verbatim", modifiers = "bar")] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", cfg(FALSE), cfg(FALSE))] // { dg-error "" "" { target *-*-* } }
#[link(wasm_import_module = "foo", wasm_import_module = "bar")] // { dg-error "" "" { target *-*-* } }
extern "C" {}

// Ill-formed arguments
#[link(name)] // { dg-error ".E0459." "" { target *-*-* } }
// { dg-error ".E0459." "" { target *-*-* } .-2 }
#[link(name())] // { dg-error ".E0459." "" { target *-*-* } }
// { dg-error ".E0459." "" { target *-*-* } .-2 }
#[link(name = "...", kind)] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", kind())] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", modifiers)] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", modifiers())] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", cfg)] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", cfg = "literal")] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", cfg("literal"))] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", wasm_import_module)] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", wasm_import_module())] // { dg-error "" "" { target *-*-* } }
extern "C" {}

// Basic modifier validation
#[link(name = "...", modifiers = "")] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", modifiers = "no-plus-minus")] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", modifiers = "+unknown")] // { dg-error "" "" { target *-*-* } }
#[link(name = "...", modifiers = "+verbatim,+verbatim")] // { dg-error "" "" { target *-*-* } }
extern "C" {}

fn main() {}

