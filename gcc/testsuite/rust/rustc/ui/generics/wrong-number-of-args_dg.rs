mod no_generics {
    struct Ty;

    type A = Ty;

    type B = Ty<'static>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    type C = Ty<'static, usize>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
// { help ".E0107." "" { target *-*-* } .-3 }
// { help ".E0107." "" { target *-*-* } .-4 }

    type D = Ty<'static, usize, { 0 }>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
// { help ".E0107." "" { target *-*-* } .-3 }
// { help ".E0107." "" { target *-*-* } .-4 }
}

mod type_and_type {
    struct Ty<A, B>(A, B);

    type A = Ty;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    type B = Ty<usize>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    type C = Ty<usize, String>;

    type D = Ty<usize, String, char>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    type E = Ty<>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
}

mod lifetime_and_type {
    struct Ty<'a, T>(&'a T);

    type A = Ty;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
// { help ".E0107." "" { target *-*-* } .-3 }
// { help ".E0107." "" { target *-*-* } .-4 }

    type B = Ty<'static>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    type C = Ty<usize>;
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { help ".E0106." "" { target *-*-* } .-2 }

    type D = Ty<'static, usize>;

    type E = Ty<>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
// { help ".E0107." "" { target *-*-* } .-3 }
// { help ".E0107." "" { target *-*-* } .-4 }

    type F = Ty<'static, usize, 'static, usize>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
// { help ".E0107." "" { target *-*-* } .-3 }
// { help ".E0107." "" { target *-*-* } .-4 }
}

mod type_and_type_and_type {
    struct Ty<A, B, C = &'static str>(A, B, C);

    type A = Ty;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    type B = Ty<usize>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    type C = Ty<usize, String>;

    type D = Ty<usize, String, char>;

    type E = Ty<usize, String, char, f64>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    type F = Ty<>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
}

// Traits have an implicit `Self` type - these tests ensure we don't accidentally return it
// somewhere in the message
mod r#trait {
    trait NonGeneric {
        //
    }

    trait GenericLifetime<'a> {
        //
    }

    trait GenericType<A> {
        //
    }

    type A = Box<dyn NonGeneric<usize>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    type B = Box<dyn GenericLifetime>;
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { help ".E0106." "" { target *-*-* } .-2 }
// { help ".E0106." "" { target *-*-* } .-3 }

    type C = Box<dyn GenericLifetime<'static, 'static>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    type D = Box<dyn GenericType>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    type E = Box<dyn GenericType<String, usize>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    type F = Box<dyn GenericLifetime<>>;
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { help ".E0106." "" { target *-*-* } .-2 }
// { help ".E0106." "" { target *-*-* } .-3 }

    type G = Box<dyn GenericType<>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
}

mod associated_item {
    mod non_generic {
        trait NonGenericAT {
            type AssocTy;
        }

        type A = Box<dyn NonGenericAT<usize, AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
    }

    mod lifetime {
        trait GenericLifetimeAT<'a> {
            type AssocTy;
        }

        type A = Box<dyn GenericLifetimeAT<AssocTy=()>>;
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { help ".E0106." "" { target *-*-* } .-2 }
// { help ".E0106." "" { target *-*-* } .-3 }

        type B = Box<dyn GenericLifetimeAT<'static, 'static, AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

        type C = Box<dyn GenericLifetimeAT<(), AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
// { help ".E0107." "" { target *-*-* } .-3 }
// { dg-error ".E0107." "" { target *-*-* } .-4 }
// { help ".E0107." "" { target *-*-* } .-5 }
    }

    mod r#type {
        trait GenericTypeAT<A> {
            type AssocTy;
        }

        type A = Box<dyn GenericTypeAT<AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

        type B = Box<dyn GenericTypeAT<(), (), AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

        type C = Box<dyn GenericTypeAT<'static, AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
// { dg-error ".E0107." "" { target *-*-* } .-3 }
// { help ".E0107." "" { target *-*-* } .-4 }
    }

    mod lifetime_and_type {
        trait GenericLifetimeTypeAT<'a, A> {
            type AssocTy;
        }

        type A = Box<dyn GenericLifetimeTypeAT<AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
// { dg-error ".E0107." "" { target *-*-* } .-3 }
// { help ".E0107." "" { target *-*-* } .-4 }
// { help ".E0107." "" { target *-*-* } .-5 }

        type B = Box<dyn GenericLifetimeTypeAT<'static, AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

        type C = Box<dyn GenericLifetimeTypeAT<'static, 'static, AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
// { dg-error ".E0107." "" { target *-*-* } .-3 }
// { help ".E0107." "" { target *-*-* } .-4 }

        type D = Box<dyn GenericLifetimeTypeAT<(), AssocTy=()>>;
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { help ".E0106." "" { target *-*-* } .-2 }
// { help ".E0106." "" { target *-*-* } .-3 }

        type E = Box<dyn GenericLifetimeTypeAT<(), (), AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
// { help ".E0107." "" { target *-*-* } .-3 }
// { dg-error ".E0107." "" { target *-*-* } .-4 }
// { help ".E0107." "" { target *-*-* } .-5 }

        type F = Box<dyn GenericLifetimeTypeAT<'static, 'static, (), AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

        type G = Box<dyn GenericLifetimeTypeAT<'static, (), (), AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

        type H = Box<dyn GenericLifetimeTypeAT<'static, 'static, (), (), AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
// { dg-error ".E0107." "" { target *-*-* } .-3 }
// { help ".E0107." "" { target *-*-* } .-4 }
    }

    mod type_and_type {
        trait GenericTypeTypeAT<A, B> {
            type AssocTy;
        }

        type A = Box<dyn GenericTypeTypeAT<AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

        type B = Box<dyn GenericTypeTypeAT<(), AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

        type C = Box<dyn GenericTypeTypeAT<(), (), (), AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
    }

    mod lifetime_and_lifetime {
        trait GenericLifetimeLifetimeAT<'a, 'b> {
            type AssocTy;
        }

        type A = Box<dyn GenericLifetimeLifetimeAT<AssocTy=()>>;
// { dg-error ".E0106." "" { target *-*-* } .-1 }
// { help ".E0106." "" { target *-*-* } .-2 }
// { help ".E0106." "" { target *-*-* } .-3 }

        type B = Box<dyn GenericLifetimeLifetimeAT<'static, AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
    }

    mod lifetime_and_lifetime_and_type {
        trait GenericLifetimeLifetimeTypeAT<'a, 'b, A> {
            type AssocTy;
        }

        type A = Box<dyn GenericLifetimeLifetimeTypeAT<AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
// { help ".E0107." "" { target *-*-* } .-3 }
// { dg-error ".E0107." "" { target *-*-* } .-4 }
// { help ".E0107." "" { target *-*-* } .-5 }

        type B = Box<dyn GenericLifetimeLifetimeTypeAT<'static, AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
// { dg-error ".E0107." "" { target *-*-* } .-3 }
// { help ".E0107." "" { target *-*-* } .-4 }

        type C = Box<dyn GenericLifetimeLifetimeTypeAT<'static, (), AssocTy=()>>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
    }
}

mod stdlib {
    mod hash_map {
        use std::collections::HashMap;

        type A = HashMap;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

        type B = HashMap<String>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

        type C = HashMap<'static>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
// { dg-error ".E0107." "" { target *-*-* } .-3 }
// { help ".E0107." "" { target *-*-* } .-4 }

        type D = HashMap<usize, String, char, f64>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

        type E = HashMap<>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
    }

    mod result {
        type A = Result;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

        type B = Result<String>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

        type C = Result<'static>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
// { dg-error ".E0107." "" { target *-*-* } .-3 }
// { help ".E0107." "" { target *-*-* } .-4 }

        type D = Result<usize, String, char>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

        type E = Result<>;
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
    }
}

fn main() { }

