//@ build-fail

#![recursion_limit = "64"]
type Byte = Option<Option<Option<Option< Option<Option<Option<Option<
    Option<Option<Option<Option< Option<Option<Option<Option<
        Option<Option<Option<Option< Option<Option<Option<Option<
            Option<Option<Option<Option< Option<Option<Option<Option<
                Option<Option<Option<Option< Option<Option<Option<Option<
                    Option<Option<Option<Option< Option<Option<Option<Option<
                        Option<Option<Option<Option< Option<Option<Option<Option<
                            Option<Option<Option<Option< Option<Option<Option<Option<
                                Option<Option<Option<Option< Option<Option<Option<Option<
                                    Option<Option<Option<Option< Option<Option<Option<Option<
                                        Option<Option<Option<Option< Option<Option<Option<Option<
                                            Box<String>
                                        >>>> >>>>
                                    >>>> >>>>
                                >>>> >>>>
                            >>>> >>>>
                        >>>> >>>>
                    >>>> >>>>
                >>>> >>>>
            >>>> >>>>
        >>>> >>>>
    >>>> >>>>
>>>> >>>>;

fn main() {
// { dg-error "" "" { target *-*-* } .-1 }
    println!("{}", std::mem::size_of::<Byte>());
}

