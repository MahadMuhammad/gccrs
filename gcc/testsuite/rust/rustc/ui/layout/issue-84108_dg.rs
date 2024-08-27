// See issue #84108 -- this is a test to ensure we do not ICE
// on this invalid code.

#![crate_type = "lib"]

static FOO: (dyn AsRef<OsStr>, u8) = ("hello", 42);
// { dg-error ".E0412." "" { target *-*-* } .-1 }

const BAR: (&Path, [u8], usize) = ("hello", [], 42);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
// { dg-error ".E0308." "" { target *-*-* } .-4 }

static BAZ: ([u8], usize) = ([], 0);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }

