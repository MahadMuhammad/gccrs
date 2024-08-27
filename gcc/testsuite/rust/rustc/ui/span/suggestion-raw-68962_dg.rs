fn r#fn() {}

fn main() {
    let r#final = 1;

    // Should correctly suggest variable defined using raw identifier.
    fina; // { dg-error ".E0425." "" { target *-*-* } }

    // Should correctly suggest function defined using raw identifier.
    f(); // { dg-error ".E0425." "" { target *-*-* } }
}

