extern "C" {
    #[used] // { dg-error "" "" { target *-*-* } }
    static FOO: i32;
}

fn main() {}

