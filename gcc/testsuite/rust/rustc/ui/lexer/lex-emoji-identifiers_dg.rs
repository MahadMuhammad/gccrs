fn invalid_emoji_usages() {
    let arrow↔️ = "basic emoji"; // { dg-error "" "" { target *-*-* } }
    let planet🪐 = "basic emoji"; // { dg-error "" "" { target *-*-* } }
    let wireless🛜 = "basic emoji"; // { dg-error "" "" { target *-*-* } }
    // FIXME
    let key1️⃣ = "keycap sequence"; // { dg-error "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-1 }
    let flag🇺🇳 = "flag sequence"; // { dg-error "" "" { target *-*-* } }
    let wales🏴 = "tag sequence"; // { dg-error "" "" { target *-*-* } }
    let folded🙏🏿 = "modifier sequence"; // { dg-error "" "" { target *-*-* } }
}

fn main() {
    invalid_emoji_usages();
}

