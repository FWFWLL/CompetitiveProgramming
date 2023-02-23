use std::collections::BTreeSet;

pub fn solution() -> String {
    let result: u32 = (3..1000)
        .step_by(3)
        .chain((5..1000).step_by(5))
        .collect::<BTreeSet<u32>>()
        .iter()
        .sum();

    return result.to_string();
}
