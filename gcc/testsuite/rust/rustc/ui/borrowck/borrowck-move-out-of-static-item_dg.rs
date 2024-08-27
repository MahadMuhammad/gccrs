// Ensure that moves out of static items is forbidden

struct Foo {
    foo: isize,
}

static BAR: Foo = Foo { foo: 5 };


fn test(f: Foo) {
    let f = Foo { foo: 4, ..f };
    println!("{}", f.foo);
}

fn main() {
    test(BAR); // { dg-error ".E0507." "" { target *-*-* } }
}

