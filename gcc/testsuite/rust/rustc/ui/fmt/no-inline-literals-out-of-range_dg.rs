//@ only-64bit

fn main() {
    format_args!("{}", 0x8f_i8); // issue #115423
// { dg-error "" "" { target *-*-* } .-1 }
    format_args!("{}", 0xffff_ffff_u8); // issue #116633
// { dg-error "" "" { target *-*-* } .-1 }
    format_args!("{}", 0xffff_ffff_ffff_ffff_ffff_usize);
// { dg-error "" "" { target *-*-* } .-1 }
    format_args!("{}", 0x8000_0000_0000_0000_isize);
// { dg-error "" "" { target *-*-* } .-1 }
    format_args!("{}", 0xffff_ffff); // treat unsuffixed literals as i32
// { dg-error "" "" { target *-*-* } .-1 }
}

