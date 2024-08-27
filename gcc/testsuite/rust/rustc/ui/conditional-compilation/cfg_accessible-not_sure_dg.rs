//@ revisions: edition2015 edition2021
//@ [edition2015]compile-flags: --edition=2015
//@ [edition2021]compile-flags: --edition=2021

#![feature(extern_types)]
#![feature(cfg_accessible)]

// Struct::unresolved - error

struct Struct {
    existing: u8,
}

#[cfg_accessible(Struct::existing)] // { dg-error "" "" { target *-*-* } }
const A: bool = true;
#[cfg_accessible(Struct::unresolved)] // { dg-error "" "" { target *-*-* } }
const B: bool = true;

// Union::unresolved - error

struct Union {
    existing: u8,
}

#[cfg_accessible(Union::existing)] // { dg-error "" "" { target *-*-* } }
const A: bool = true;
#[cfg_accessible(Union::unresolved)] // { dg-error "" "" { target *-*-* } }
const B: bool = true;

// Enum::unresolved - error

enum Enum {
    Existing { existing: u8 },
}

#[cfg_accessible(Enum::Existing::existing)] // { dg-error "" "" { target *-*-* } }
const A: bool = true;
#[cfg_accessible(Enum::Existing::unresolved)] // { dg-error "" "" { target *-*-* } }
const B: bool = true;
#[cfg_accessible(Enum::unresolved)] // { dg-error "" "" { target *-*-* } }
const C: bool = true;

// Trait::unresolved - false or error, depending on edition (error if you can write Trait::foo
// instead of <dyn Trait>::foo for methods like impl dyn Trait { fn foo() {} })

trait Trait {}
impl dyn Trait { fn existing() {} }

// FIXME: Should be an error for edition > 2015
#[cfg_accessible(Trait::existing)] // { dg-error "" "" { target *-*-* } }
const A: bool = true;
#[cfg_accessible(Trait::unresolved)] // { dg-error "" "" { target *-*-* } }
const B: bool = true;

// TypeAlias::unresolved - error

type TypeAlias = Struct;

#[cfg_accessible(TypeAlias::existing)] // { dg-error "" "" { target *-*-* } }
const A: bool = true;
#[cfg_accessible(TypeAlias::unresolved)] // { dg-error "" "" { target *-*-* } }
const B: bool = true;

// ForeignType::unresolved - error

extern {
    type ForeignType;
}

#[cfg_accessible(ForeignType::unresolved)] // { dg-error "" "" { target *-*-* } }
const A: bool = true;

// AssocType::unresolved - error

trait AssocType {
    type AssocType;
}

#[cfg_accessible(AssocType::AssocType::unresolved)] // { dg-error "" "" { target *-*-* } }
const A: bool = true;

// PrimitiveType::unresolved - error

#[cfg_accessible(u8::unresolved)] // { dg-error "" "" { target *-*-* } }
const A: bool = true;
#[cfg_accessible(u8::is_ascii)] // { dg-error "" "" { target *-*-* } }
const B: bool = true;

fn main() {}

