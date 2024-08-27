fn main() {}

struct CLI {
    #[derive(parse())] // { dg-error "" "" { target *-*-* } }
    path: (),
}

