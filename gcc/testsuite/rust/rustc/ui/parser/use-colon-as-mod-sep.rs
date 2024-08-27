// Recover from using a colon as a path separator.

use std::process:Command;
// { dg-error "" "" { target *-*-* } .-1 }
use std:fs::File;
// { dg-error "" "" { target *-*-* } .-1 }
use std:collections:HashMap;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

fn main() { }

