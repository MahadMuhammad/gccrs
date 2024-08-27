// ICE argument to function with "rust-call" ABI is not a tuple
// issue: rust-lang/rust#81974

#![feature(unboxed_closures)]
#![feature(fn_traits)]

use std::collections::HashMap;
use std::hash::Hash;

struct CachedFun<A, B> {
    cache: HashMap<A, B>,
    fun: fn(&mut CachedFun<A, B>, A) -> B,
}

impl<A: Eq + Hash, B> CachedFun<A, B> {
    fn new(fun: fn(&mut Self, A) -> B) -> Self {
        CachedFun {
            cache: HashMap::new(),
            fun,
        }
    }
}

impl<A, B> FnOnce<A> for CachedFun<A, B>
// { dg-error ".E0059." "" { target *-*-* } .-1 }
where
    A: Eq + Hash + Clone,
    B: Clone,
{
    type Output = B;
    extern "rust-call" fn call_once(mut self, a: A) -> Self::Output {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        self.call_mut(a)
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    }
}

impl<A, B> FnMut<A> for CachedFun<A, B>
// { dg-error ".E0059." "" { target *-*-* } .-1 }
where
    A: Eq + Hash + Clone,
    B: Clone,
{
    extern "rust-call" fn call_mut(&mut self, a: A) -> Self::Output {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        self.cache.get(&a).map(|a| a.clone()).unwrap_or_else(|| {
            let b = (self.fun)(self, a.clone());
            self.cache.insert(a, b.clone());
            b
        })
    }
}

fn main() -> () {
    let pesce = |y: &mut CachedFun<i32, i32>, x| x + 1;
    let cachedcoso = CachedFun::new(pesce);
    cachedcoso.call_once(1);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

