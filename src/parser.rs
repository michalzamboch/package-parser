use std::collections::HashSet;

pub fn parse_packages(input: &str) -> Vec<&str> {
    let mut result = filter_all_packages(input).into_iter().collect::<Vec<&str>>();
    result.sort();
    result
}

fn filter_all_packages(input: &str) -> HashSet<&str> {
    input
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| l.starts_with(">"))
        .map(get_package_from_line)
        .filter(|l| !l.is_empty())
        .collect::<HashSet<&str>>()
}

fn get_package_from_line(line: &str) -> &str {
    let result  = line
        .split_whitespace()
        .collect::<Vec<&str>>();

    match result.get(1) {
        Some(x) => x,
        None => "",
    }
}