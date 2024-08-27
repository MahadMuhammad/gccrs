fn main() {
    foo<<S as T>::V>(); // { dg-error "" "" { target *-*-* } }
}

