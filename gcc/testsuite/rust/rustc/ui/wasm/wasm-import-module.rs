#![feature(link_cfg)]

#[link(name = "...", wasm_import_module)] // { dg-error "" "" { target *-*-* } }
extern "C" {}

#[link(name = "...", wasm_import_module(x))] // { dg-error "" "" { target *-*-* } }
extern "C" {}

#[link(name = "...", wasm_import_module())] // { dg-error "" "" { target *-*-* } }
extern "C" {}

#[link(wasm_import_module = "foo", name = "bar")] // { dg-error "" "" { target *-*-* } }
extern "C" {}

#[link(wasm_import_module = "foo", kind = "dylib")] // { dg-error "" "" { target *-*-* } }
extern "C" {}

#[link(wasm_import_module = "foo", cfg(FALSE))] // { dg-error "" "" { target *-*-* } }
extern "C" {}

fn main() {}

