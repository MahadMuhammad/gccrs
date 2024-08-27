//@ only-aarch64
//@ build-fail
//@ needs-asm-support
//@ compile-flags: -Ccodegen-units=1

use std::arch::asm;

// Checks that inline asm errors are mapped to the correct line in the source code.

fn main() {
    unsafe {
        asm!("invalid_instruction");
// { dg-error "" "" { target *-*-* } .-1 }

        asm!("
            invalid_instruction
        ");
// { dg-error "" "" { target *-*-* } .-2 }

        asm!(r#"
            invalid_instruction
        "#);
// { dg-error "" "" { target *-*-* } .-2 }

        asm!("
            mov x0, x0
            invalid_instruction
            mov x0, x0
        ");
// { dg-error "" "" { target *-*-* } .-3 }

        asm!(r#"
            mov x0, x0
            invalid_instruction
            mov x0, x0
        "#);
// { dg-error "" "" { target *-*-* } .-3 }

        asm!(concat!("invalid", "_", "instruction"));
// { dg-error "" "" { target *-*-* } .-1 }

        asm!(
            "invalid_instruction",
        );
// { dg-error "" "" { target *-*-* } .-2 }

        asm!(
            "mov x0, x0",
            "invalid_instruction",
            "mov x0, x0",
        );
// { dg-error "" "" { target *-*-* } .-3 }

        asm!(
            "mov x0, x0\n",
            "invalid_instruction",
            "mov x0, x0",
        );
// { dg-error "" "" { target *-*-* } .-3 }

        asm!(
            "mov x0, x0",
            concat!("invalid", "_", "instruction"),
            "mov x0, x0",
        );
// { dg-error "" "" { target *-*-* } .-3 }

        asm!(
            concat!("mov x0", ", ", "x0"),
            concat!("invalid", "_", "instruction"),
            concat!("mov x0", ", ", "x0"),
        );
// { dg-error "" "" { target *-*-* } .-3 }

        // Make sure template strings get separated
        asm!(
            "invalid_instruction1",
            "invalid_instruction2",
        );
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-3 }

        asm!(
            concat!(
                "invalid", "_", "instruction1", "\n",
                "invalid", "_", "instruction2",
            ),
        );
// { dg-error "" "" { target *-*-* } .-5 }
// { dg-error "" "" { target *-*-* } .-6 }

        asm!(
            concat!(
                "invalid", "_", "instruction1", "\n",
                "invalid", "_", "instruction2",
            ),
            concat!(
                "invalid", "_", "instruction3", "\n",
                "invalid", "_", "instruction4",
            ),
        );
// { dg-error "" "" { target *-*-* } .-9 }
// { dg-error "" "" { target *-*-* } .-10 }
// { dg-error "" "" { target *-*-* } .-7 }
// { dg-error "" "" { target *-*-* } .-8 }

        asm!(
            concat!(
                "invalid", "_", "instruction1", "\n",
                "invalid", "_", "instruction2", "\n",
            ),
            concat!(
                "invalid", "_", "instruction3", "\n",
                "invalid", "_", "instruction4", "\n",
            ),
        );
// { dg-error "" "" { target *-*-* } .-9 }
// { dg-error "" "" { target *-*-* } .-10 }
// { dg-error "" "" { target *-*-* } .-7 }
// { dg-error "" "" { target *-*-* } .-8 }

        asm!(
            "",
            "\n",
            "invalid_instruction"
        );
// { dg-error "" "" { target *-*-* } .-2 }
    }
}

