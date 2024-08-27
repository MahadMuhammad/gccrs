macro_rules! many_args {
    ([$($t:tt)*]#$($h:tt)*) => {
        many_args!{[$($t)*$($t)*]$($h)*}
    };
    ([$($t:tt)*]) => {
        fn _f($($t: ()),*) {} // { dg-error "" "" { target *-*-* } }
    }
}

many_args!{[_]########## ######}

fn main() {}

