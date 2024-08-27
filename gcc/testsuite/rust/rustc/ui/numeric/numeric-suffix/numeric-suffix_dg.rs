//@ run-rustfix

fn foo<N>(_x: N) {}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }
// { dg-note "" "" { target *-*-* } .-7 }
// { dg-note "" "" { target *-*-* } .-8 }
// { dg-note "" "" { target *-*-* } .-9 }
// { dg-note "" "" { target *-*-* } .-10 }
// { dg-note "" "" { target *-*-* } .-11 }
// { dg-note "" "" { target *-*-* } .-12 }
// { dg-note "" "" { target *-*-* } .-13 }
// { dg-note "" "" { target *-*-* } .-14 }
// { dg-note "" "" { target *-*-* } .-15 }
// { dg-note "" "" { target *-*-* } .-16 }
// { dg-note "" "" { target *-*-* } .-17 }
// { dg-note "" "" { target *-*-* } .-18 }
// { dg-note "" "" { target *-*-* } .-19 }
// { dg-note "" "" { target *-*-* } .-20 }
// { dg-note "" "" { target *-*-* } .-21 }
// { dg-note "" "" { target *-*-* } .-22 }
// { dg-note "" "" { target *-*-* } .-23 }
// { dg-note "" "" { target *-*-* } .-24 }
// { dg-note "" "" { target *-*-* } .-25 }
// { dg-note "" "" { target *-*-* } .-26 }
// { dg-note "" "" { target *-*-* } .-27 }
// { dg-note "" "" { target *-*-* } .-28 }
// { dg-note "" "" { target *-*-* } .-29 }
// { dg-note "" "" { target *-*-* } .-30 }
// { dg-note "" "" { target *-*-* } .-31 }
// { dg-note "" "" { target *-*-* } .-32 }
// { dg-note "" "" { target *-*-* } .-33 }
// { dg-note "" "" { target *-*-* } .-34 }
// { dg-note "" "" { target *-*-* } .-35 }
// { dg-note "" "" { target *-*-* } .-36 }
// { dg-note "" "" { target *-*-* } .-37 }
// { dg-note "" "" { target *-*-* } .-38 }
// { dg-note "" "" { target *-*-* } .-39 }
// { dg-note "" "" { target *-*-* } .-40 }
// { dg-note "" "" { target *-*-* } .-41 }
// { dg-note "" "" { target *-*-* } .-42 }
// { dg-note "" "" { target *-*-* } .-43 }
// { dg-note "" "" { target *-*-* } .-44 }
// { dg-note "" "" { target *-*-* } .-45 }
// { dg-note "" "" { target *-*-* } .-46 }
// { dg-note "" "" { target *-*-* } .-47 }
// { dg-note "" "" { target *-*-* } .-48 }
// { dg-note "" "" { target *-*-* } .-49 }
// { dg-note "" "" { target *-*-* } .-50 }
// { dg-note "" "" { target *-*-* } .-51 }
// { dg-note "" "" { target *-*-* } .-52 }
// { dg-note "" "" { target *-*-* } .-53 }
// { dg-note "" "" { target *-*-* } .-54 }
// { dg-note "" "" { target *-*-* } .-55 }
// { dg-note "" "" { target *-*-* } .-56 }
// { dg-note "" "" { target *-*-* } .-57 }
// { dg-note "" "" { target *-*-* } .-58 }
// { dg-note "" "" { target *-*-* } .-59 }
// { dg-note "" "" { target *-*-* } .-60 }
// { dg-note "" "" { target *-*-* } .-61 }
// { dg-note "" "" { target *-*-* } .-62 }
// { dg-note "" "" { target *-*-* } .-63 }
// { dg-note "" "" { target *-*-* } .-64 }
// { dg-note "" "" { target *-*-* } .-65 }
// { dg-note "" "" { target *-*-* } .-66 }
// { dg-note "" "" { target *-*-* } .-67 }
// { dg-note "" "" { target *-*-* } .-68 }
// { dg-note "" "" { target *-*-* } .-69 }
// { dg-note "" "" { target *-*-* } .-70 }
// { dg-note "" "" { target *-*-* } .-71 }
// { dg-note "" "" { target *-*-* } .-72 }
// { dg-note "" "" { target *-*-* } .-73 }
// { dg-note "" "" { target *-*-* } .-74 }
// { dg-note "" "" { target *-*-* } .-75 }
// { dg-note "" "" { target *-*-* } .-76 }
// { dg-note "" "" { target *-*-* } .-77 }
// { dg-note "" "" { target *-*-* } .-78 }
// { dg-note "" "" { target *-*-* } .-79 }
// { dg-note "" "" { target *-*-* } .-80 }
// { dg-note "" "" { target *-*-* } .-81 }
// { dg-note "" "" { target *-*-* } .-82 }
// { dg-note "" "" { target *-*-* } .-83 }
// { dg-note "" "" { target *-*-* } .-84 }
// { dg-note "" "" { target *-*-* } .-85 }
// { dg-note "" "" { target *-*-* } .-86 }
// { dg-note "" "" { target *-*-* } .-87 }
// { dg-note "" "" { target *-*-* } .-88 }
// { dg-note "" "" { target *-*-* } .-89 }
// { dg-note "" "" { target *-*-* } .-90 }
// { dg-note "" "" { target *-*-* } .-91 }
// { dg-note "" "" { target *-*-* } .-92 }
// { dg-note "" "" { target *-*-* } .-93 }
// { dg-note "" "" { target *-*-* } .-94 }
// { dg-note "" "" { target *-*-* } .-95 }
// { dg-note "" "" { target *-*-* } .-96 }
// { dg-note "" "" { target *-*-* } .-97 }
// { dg-note "" "" { target *-*-* } .-98 }
// { dg-note "" "" { target *-*-* } .-99 }
// { dg-note "" "" { target *-*-* } .-100 }
// { dg-note "" "" { target *-*-* } .-101 }
// { dg-note "" "" { target *-*-* } .-102 }
// { dg-note "" "" { target *-*-* } .-103 }
// { dg-note "" "" { target *-*-* } .-104 }
// { dg-note "" "" { target *-*-* } .-105 }
// { dg-note "" "" { target *-*-* } .-106 }
// { dg-note "" "" { target *-*-* } .-107 }
// { dg-note "" "" { target *-*-* } .-108 }
// { dg-note "" "" { target *-*-* } .-109 }
// { dg-note "" "" { target *-*-* } .-110 }
// { dg-note "" "" { target *-*-* } .-111 }
// { dg-note "" "" { target *-*-* } .-112 }
// { dg-note "" "" { target *-*-* } .-113 }
// { dg-note "" "" { target *-*-* } .-114 }
// { dg-note "" "" { target *-*-* } .-115 }
// { dg-note "" "" { target *-*-* } .-116 }
// { dg-note "" "" { target *-*-* } .-117 }
// { dg-note "" "" { target *-*-* } .-118 }
// { dg-note "" "" { target *-*-* } .-119 }
// { dg-note "" "" { target *-*-* } .-120 }
// { dg-note "" "" { target *-*-* } .-121 }
// { dg-note "" "" { target *-*-* } .-122 }
// { dg-note "" "" { target *-*-* } .-123 }
// { dg-note "" "" { target *-*-* } .-124 }
// { dg-note "" "" { target *-*-* } .-125 }
// { dg-note "" "" { target *-*-* } .-126 }
// { dg-note "" "" { target *-*-* } .-127 }
// { dg-note "" "" { target *-*-* } .-128 }
// { dg-note "" "" { target *-*-* } .-129 }
// { dg-note "" "" { target *-*-* } .-130 }
// { dg-note "" "" { target *-*-* } .-131 }
// { dg-note "" "" { target *-*-* } .-132 }
// { dg-note "" "" { target *-*-* } .-133 }
// { dg-note "" "" { target *-*-* } .-134 }
// { dg-note "" "" { target *-*-* } .-135 }
// { dg-note "" "" { target *-*-* } .-136 }


fn main() {
    foo::<u16>(42_usize);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u16>(42_u64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u16>(42_u32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u16>(42_u16);
    foo::<u16>(42_u8);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u16>(42_isize);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u16>(42_i64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u16>(42_i32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u16>(42_i16);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u16>(42_i8);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u16>(42.0_f64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u16>(42.0_f32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }

    foo::<i16>(42_usize);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i16>(42_u64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i16>(42_u32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i16>(42_u16);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i16>(42_u8);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i16>(42_isize);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i16>(42_i64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i16>(42_i32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i16>(42_i16);
    foo::<i16>(42_i8);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i16>(42.0_f64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i16>(42.0_f32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }

    foo::<u8>(42_usize);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u8>(42_u64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u8>(42_u32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u8>(42_u16);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u8>(42_u8);
    foo::<u8>(42_isize);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u8>(42_i64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u8>(42_i32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u8>(42_i16);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u8>(42_i8);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u8>(42.0_f64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<u8>(42.0_f32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }

    foo::<i8>(42_usize);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i8>(42_u64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i8>(42_u32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i8>(42_u16);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i8>(42_u8);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i8>(42_isize);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i8>(42_i64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i8>(42_i32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i8>(42_i16);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i8>(42_i8);
    foo::<i8>(42.0_f64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i8>(42.0_f32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }

    foo::<f64>(42_usize);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f64>(42_u64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f64>(42_u32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f64>(42_u16);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f64>(42_u8);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f64>(42_isize);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f64>(42_i64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f64>(42_i32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f64>(42_i16);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f64>(42_i8);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f64>(42.0_f64);
    foo::<f64>(42.0_f32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }

    foo::<f32>(42_usize);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f32>(42_u64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f32>(42_u32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f32>(42_u16);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f32>(42_u8);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f32>(42_isize);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f32>(42_i64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f32>(42_i32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f32>(42_i16);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f32>(42_i8);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f32>(42.0_f64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<f32>(42.0_f32);

    foo::<u32>(42_u8 as u16);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<i32>(-42_i8);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
}

