fn main() {
    let _: UnknownStruct<7>;
// { dg-error ".E0412." "" { target *-*-* } .-1 }
}

