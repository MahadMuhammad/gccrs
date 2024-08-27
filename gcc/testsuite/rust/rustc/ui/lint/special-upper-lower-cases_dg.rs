// (#77273) These characters are in the general categories of
// "Uppercase/Lowercase Letter".
// The diagnostics don't provide meaningful suggestions for them
// as we cannot convert them properly.

//@ check-pass

#![allow(uncommon_codepoints, unused)]

struct 𝕟𝕠𝕥𝕒𝕔𝕒𝕞𝕖𝕝;
// { dg-warning "" "" { target *-*-* } .-1 }

// FIXME: How we should handle this?
struct 𝕟𝕠𝕥_𝕒_𝕔𝕒𝕞𝕖𝕝;
// { dg-warning "" "" { target *-*-* } .-1 }

static 𝗻𝗼𝗻𝘂𝗽𝗽𝗲𝗿𝗰𝗮𝘀𝗲: i32 = 1;
// { dg-warning "" "" { target *-*-* } .-1 }

fn main() {
    let 𝓢𝓝𝓐𝓐𝓐𝓐𝓚𝓔𝓢 = 1;
// { dg-warning "" "" { target *-*-* } .-1 }
}

