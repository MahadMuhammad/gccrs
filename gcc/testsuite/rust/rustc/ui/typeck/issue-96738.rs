fn main() {
    Some.nonexistent_method(); // { dg-error ".E0599." "" { target *-*-* } }
    Some.nonexistent_field; // { dg-error ".E0609." "" { target *-*-* } }
}

