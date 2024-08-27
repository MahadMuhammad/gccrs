fn main() {
    enum Test {
        Var1,
        Var2(String),
        Var3 {
            abc: {}, // { dg-error "" "" { target *-*-* } }
        },
    }

    // recover...
    let () = 1; // { dg-error ".E0308." "" { target *-*-* } }
    enum Test2 {
        Fine,
    }

    enum Test3 {
        StillFine {
            def: i32,
        },
    }

    {
        // fail again
        enum Test4 {
            Nope(i32 {}) // { dg-error "" "" { target *-*-* } }
        }
        let () = 1; // { dg-error ".E0308." "" { target *-*-* } }
    }
}

