//@ normalize-stderr-test: "`: .*" -> "`: $$FILE_NOT_FOUND_MSG"

fn main() {
    let _ = include_str!("include-macros/file.txt");            // { dg-error "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
    let _ = include_str!("hello.rs");                           // { dg-error "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
    let _ = include_bytes!("../../data.bin");                   // { dg-error "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
    let _ = include_str!("tests/ui/include-macros/file.txt");   // { dg-error "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
}

