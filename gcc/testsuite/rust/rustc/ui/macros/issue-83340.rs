//@ check-fail

fn main() {
    println!(
        "\
\n {} │", // { dg-error "" "" { target *-*-* } }
    );
}

