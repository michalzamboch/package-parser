use std::collections::HashSet;

pub fn parse_packages(input: &str) -> HashSet<&str> {
    input
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| l.starts_with(">"))
        .map(|l| {
            let result: Vec<&str> = l.split_whitespace().collect();
            let package = result.get(1);

            match package {
                Some(x) => x,
                None => "",
            }
        })
        .filter(|l| !l.is_empty())
        .collect::<HashSet<&str>>()
}
