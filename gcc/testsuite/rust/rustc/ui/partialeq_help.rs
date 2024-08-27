fn foo<T: PartialEq>(a: &T, b: T) {
    a == b; // { dg-error ".E0277." "" { target *-*-* } }
}

fn foo2<T: PartialEq>(a: &T, b: T) where {
    a == b; // { dg-error ".E0277." "" { target *-*-* } }
}

fn main() {
    foo(&1, 1);
    foo2(&1, 1);
}

