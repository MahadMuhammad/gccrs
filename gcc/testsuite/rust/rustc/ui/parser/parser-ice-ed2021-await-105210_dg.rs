// ICE #105210 self.lines.iter().all(|r| !r.iter().any(|sc| sc.chr == \'\\t\'))
// ignore-tidy-tab
// { dg-additional-options "-frust-edition=2021" }
pub fn main() {}

fn box () {
 (( h (const {( default ( await ( await (	(move {await((((}}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } }

