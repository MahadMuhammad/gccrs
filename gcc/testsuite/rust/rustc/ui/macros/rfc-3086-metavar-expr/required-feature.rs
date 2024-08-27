macro_rules! count {
    ( $( $e:stmt ),* ) => {
        ${ count($e) }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    };
}

macro_rules! dollar_dollar {
    () => {
        macro_rules! bar {
            ( $$( $$any:tt )* ) => { $$( $$any )* };
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
// { dg-error ".E0658." "" { target *-*-* } .-3 }
// { dg-error ".E0658." "" { target *-*-* } .-4 }
        }
    };
}

macro_rules! index {
    ( $( $e:stmt ),* ) => {
        $( ${ignore($e)} ${index()} )*
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
    };
}

macro_rules! ignore {
    ( $( $i:stmt ),* ) => {{
        0 $( + 1 ${ignore($i)} )*
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    }};
}

macro_rules! len {
    ( $( $e:stmt ),* ) => {
        $( ${ignore($e)} ${len()} )*
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
    };
}

fn main() {}

