mod m {
    pub const P: usize = 0;
}

const Q: usize = 0;

fn test<const N: usize>() {
    struct Foo<const M: usize>;
    macro_rules! foo {
        ($x:expr) => {
            [u8; $x]
        }
    }
    macro_rules! bar {
        ($x:expr) => {
            [u8; { $x }]
        }
    }
    macro_rules! baz {
        ( $x:expr) => {
            Foo<$x>
        }
    }
    macro_rules! biz {
        ($x:expr) => {
            Foo<{ $x }>
        };
    }

    let _: foo!(N);
    let _: foo!({ N });
    let _: foo!({{ N }}); // { dg-error "" "" { target *-*-* } }
    let _: foo!(Q);
    let _: foo!(m::P);
    let _: bar!(N);
    let _: bar!({ N }); // { dg-error "" "" { target *-*-* } }
    let _: bar!(Q);
    let _: bar!(m::P);
    let _: baz!(N);
    let _: baz!({ N });
    let _: baz!({{ N }}); // { dg-error "" "" { target *-*-* } }
    let _: baz!(Q);
    let _: baz!({ m::P });
    let _: baz!(m::P); // { dg-error "" "" { target *-*-* } }
    let _: biz!(N);
    let _: biz!({ N }); // { dg-error "" "" { target *-*-* } }
    let _: biz!(Q);
    let _: biz!(m::P);
    let _: foo!(3);
    let _: foo!({ 3 });
    let _: foo!({{ 3 }});
    let _: bar!(3);
    let _: bar!({ 3 });
    let _: baz!(3);
    let _: baz!({ 3 });
    let _: baz!({{ 3 }});
    let _: biz!(3);
    let _: biz!({ 3 });
    let _: foo!(10 + 7);
    let _: foo!({ 10 + 7 });
    let _: foo!({{ 10 + 7 }});
    let _: bar!(10 + 7);
    let _: bar!({ 10 + 7 });
    let _: baz!(10 + 7); // { dg-error "" "" { target *-*-* } }
    let _: baz!({ 10 + 7 });
    let _: baz!({{ 10 + 7 }});
    let _: biz!(10 + 7);
    let _: biz!({ 10 + 7 });
}

fn main() {
    test::<3>();
}

