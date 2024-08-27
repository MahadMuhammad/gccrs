const FOO: usize = 0;

fn main() {
    FOO(); // { dg-error ".E0618." "" { target *-*-* } }
}

