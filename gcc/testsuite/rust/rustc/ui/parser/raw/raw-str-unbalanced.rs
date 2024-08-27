static s: &'static str =
    r#""## // { dg-error "" "" { target *-*-* } }
;

static s2: &'static str =
    r#"
      "#### // { dg-error "" "" { target *-*-* } }
;

const A: &'static str = r"" // { dg-error "" "" { target *-*-* } }

// Test
#[test]
fn test() {}

const B: &'static str = r""## // { dg-error "" "" { target *-*-* } }

// Test
#[test]
fn test2() {}

fn main() {}

