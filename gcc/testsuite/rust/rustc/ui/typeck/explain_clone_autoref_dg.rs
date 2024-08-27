struct NotClone;

fn main() {
    clone_thing(&NotClone);
}

fn clone_thing(nc: &NotClone) -> NotClone {
// { dg-note "" "" { target *-*-* } .-1 }
    nc.clone()
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
}

fn clone_thing2(nc: &NotClone) -> NotClone {
    let nc: NotClone = nc.clone();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
// { dg-note ".E0308." "" { target *-*-* } .-4 }
    nc
}

fn clone_thing3(nc: &NotClone) -> NotClone {
// { dg-note "" "" { target *-*-* } .-1 }
    let nc = nc.clone();
// { dg-note "" "" { target *-*-* } .-1 }
    nc
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
}

fn clone_thing4(nc: &NotClone) -> NotClone {
// { dg-note "" "" { target *-*-* } .-1 }
    let nc = nc.clone();
// { dg-note "" "" { target *-*-* } .-1 }
    let nc2 = nc;
    nc2
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
}

impl NotClone {
    fn other_fn(&self) {}
    fn get_ref_notclone(&self) -> &Self {
        self
    }
}

fn clone_thing5(nc: &NotClone) -> NotClone {
// { dg-note "" "" { target *-*-* } .-1 }
    let nc = nc.clone();
// { dg-note "" "" { target *-*-* } .-1 }
    let nc2 = nc;
    nc2.other_fn();
    let nc3 = nc2;
    nc3
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
}

fn clone_thing6(nc: &NotClone) -> NotClone {
// { dg-note "" "" { target *-*-* } .-1 }
    let (ret, _) = (nc.clone(), 1);
// { dg-note "" "" { target *-*-* } .-1 }
    let _ = nc.clone();
    ret
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
}

fn clone_thing7(nc: Vec<&NotClone>) -> NotClone {
// { dg-note "" "" { target *-*-* } .-1 }
    let ret = nc[0].clone();
// { dg-note "" "" { target *-*-* } .-1 }
    ret
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
}

fn clone_thing8(nc: &NotClone) -> NotClone {
// { dg-note "" "" { target *-*-* } .-1 }
    let ret = {
        let a = nc.clone();
// { dg-note "" "" { target *-*-* } .-1 }
        a
    };
    ret
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
}

fn clone_thing9(nc: &NotClone) -> NotClone {
// { dg-note "" "" { target *-*-* } .-1 }
    let cl = || nc.clone();
// { dg-note "" "" { target *-*-* } .-1 }
    let ret = cl();
    ret
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
}

fn clone_thing10(nc: &NotClone) -> (NotClone, NotClone) {
    let (a, b) = {
        let a = nc.clone();
// { dg-note "" "" { target *-*-* } .-1 }
        (a, nc.clone())
// { dg-note "" "" { target *-*-* } .-1 }
    };
    (a, b)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
// { dg-note ".E0308." "" { target *-*-* } .-4 }
}

fn clone_thing11(nc: &NotClone) -> NotClone {
// { dg-note "" "" { target *-*-* } .-1 }
    let a = {
        let nothing = nc.clone();
        let a = nc.clone();
// { dg-note "" "" { target *-*-* } .-1 }
        let nothing = nc.clone();
        a
    };
    a
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
}

