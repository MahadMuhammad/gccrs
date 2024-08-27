fn a() {
    [0; [|_: _ &_| ()].len()]
// { dg-error ".E0282." "" { target *-*-* } .-1 }
// { dg-error ".E0282." "" { target *-*-* } .-2 }
}

fn b() {
    [0; [|f @ &ref _| {} ; 0 ].len() ];
// { dg-error "" "" { target *-*-* } .-1 }
}

fn c() {
    [0; [|&_: _ &_| {}; 0 ].len()]
// { dg-error ".E0282." "" { target *-*-* } .-1 }
// { dg-error ".E0282." "" { target *-*-* } .-2 }
}

fn d() {
    [0; match [|f @ &ref _| () ] {} ]
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

