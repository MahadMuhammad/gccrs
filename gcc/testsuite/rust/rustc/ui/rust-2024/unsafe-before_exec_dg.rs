//@ revisions: e2021 e2024
//@ only-unix
// { dg-additional-options "-frust-edition= 2021" }
//@[e2021] check-pass
// { dg-additional-options "-frust-edition= 2024" }
//@[e2024] compile-flags: -Zunstable-options

use std::process::Command;
use std::os::unix::process::CommandExt;

#[allow(deprecated)]
fn main() {
    let mut cmd = Command::new("sleep");
    cmd.before_exec(|| Ok(()));
// { dg-error "" "" { target *-*-* } .-1 }
    drop(cmd); // we don't actually run the command.
}

