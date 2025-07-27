#![allow(dead_code)]

use std::collections::*;

pub fn create_package_map(packages: Vec<&str>) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for package in packages {
        let row = package.split(".").collect::<Vec<&str>>();

        match map.get_mut(row[0]) {
            Some(value) => value.push(package),
            None => _ = map.insert(row[0], vec![package]),
        };
    }

    map
}

pub fn create_package_table(packages: Vec<&str>) -> Vec<Vec<&str>> {
    let mut table: Vec<Vec<&str>> = vec![];
    
    for package in packages {
        let row = package.split(".").collect::<Vec<&str>>();
        table.push(row);
    }

    table
}

pub fn parse_packages(input: &str) -> Vec<&str> {
    let mut result = filter_all_packages(input)
        .into_iter()
        .collect::<Vec<&str>>();

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