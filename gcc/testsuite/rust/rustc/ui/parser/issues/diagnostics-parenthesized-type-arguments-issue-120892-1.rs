fn main() {
  foo::( // { help "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
    bar(x, y, z),
    bar(x, y, z),
    bar(x, y, z),
    bar(x, y, z),
    bar(x, y, z),
    bar(x, y, z),
    bar(x, y, z),
    baz("test"), // { dg-error "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
  )
}

