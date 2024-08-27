// { dg-additional-options "-frust-edition=2021" }
//@ aux-build:issue-107113.rs

#[macro_use]
extern crate issue_107113;

#[issue_107113::main] // { dg-error ".E0308." "" { target *-*-* } }
async fn main() -> std::io::Result<()> {}

