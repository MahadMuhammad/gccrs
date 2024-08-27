trait TypeMapper {
    type MapType;
}

type Mapped<T> = <T as TypeMapper>::MapType;

struct Test {}

impl TypeMapper for () {
    type MapType = Test;
}

fn test() {
    Mapped::<()> {};
    None; // { dg-error ".E0282." "" { target *-*-* } }
}

fn main() {}

