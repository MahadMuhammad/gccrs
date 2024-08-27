struct Process;

pub type Group = (Vec<String>, Vec<Process>);

fn test(process: &Process, groups: Vec<Group>) -> Vec<Group> {
    let new_group = vec![String::new()];

    if groups.capacity() == 0 {
        groups.push(new_group, vec![process]);
// { dg-error ".E0061." "" { target *-*-* } .-1 }
        return groups;
    }

    todo!()
}

fn main() {}

