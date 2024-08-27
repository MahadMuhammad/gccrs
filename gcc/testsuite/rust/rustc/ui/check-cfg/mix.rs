// This test checks the combination of well known names, the usage of cfg(),
// and that no implicit cfgs is added from --cfg while also testing that
// we correctly lint on the `cfg!` macro and `cfg_attr` attribute.
//
//@ check-pass
//@ no-auto-check-cfg
//@ compile-flags: --cfg feature="bar" --cfg unknown_name
//@ compile-flags: --check-cfg=cfg(feature,values("foo"))

#[cfg(windows)]
fn do_windows_stuff() {}

#[cfg(widnows)]
// { dg-warning "" "" { target *-*-* } .-1 }
fn do_windows_stuff() {}

#[cfg(feature)]
// { dg-warning "" "" { target *-*-* } .-1 }
fn no_feature() {}

#[cfg(feature = "foo")]
fn use_foo() {}

#[cfg(feature = "bar")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn use_bar() {}

#[cfg(feature = "zebra")]
// { dg-warning "" "" { target *-*-* } .-1 }
fn use_zebra() {}

#[cfg_attr(uu, test)]
// { dg-warning "" "" { target *-*-* } .-1 }
fn do_test() {}

#[cfg_attr(feature = "foo", no_mangle)]
fn do_test_foo() {}

fn test_cfg_macro() {
    cfg!(windows);
    cfg!(widnows);
// { dg-warning "" "" { target *-*-* } .-1 }
    cfg!(feature = "foo");
    cfg!(feature = "bar");
// { dg-warning "" "" { target *-*-* } .-1 }
    cfg!(feature = "zebra");
// { dg-warning "" "" { target *-*-* } .-1 }
    cfg!(xxx = "foo");
// { dg-warning "" "" { target *-*-* } .-1 }
    cfg!(xxx);
// { dg-warning "" "" { target *-*-* } .-1 }
    cfg!(any(xxx, windows));
// { dg-warning "" "" { target *-*-* } .-1 }
    cfg!(any(feature = "bad", windows));
// { dg-warning "" "" { target *-*-* } .-1 }
    cfg!(any(windows, xxx));
// { dg-warning "" "" { target *-*-* } .-1 }
    cfg!(all(unix, xxx));
// { dg-warning "" "" { target *-*-* } .-1 }
    cfg!(all(aa, bb));
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    cfg!(any(aa, bb));
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    cfg!(any(unix, feature = "zebra"));
// { dg-warning "" "" { target *-*-* } .-1 }
    cfg!(any(xxx, feature = "zebra"));
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    cfg!(any(xxx, unix, xxx));
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    cfg!(all(feature = "zebra", feature = "zebra", feature = "zebra"));
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
    cfg!(target_feature = "zebra");
// { dg-warning "" "" { target *-*-* } .-1 }
}

fn main() {}

