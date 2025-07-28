#![allow(dead_code)]

use std::collections::*;

pub fn create_package_patterns(packages: Vec<String>) -> Vec<String> {
    packages
        .iter()
        .map(|p| format!("<package pattern=\"{p}\" />").to_string())
        .collect::<Vec<String>>()
}

pub fn create_masked_packages(map: HashMap<&str, Vec<&str>>) -> Vec<String> {
    let mut packages: Vec<String> = vec![];

    for (k, v) in map.iter() {
        if v.len() == 0 {
            continue;
        }

        if v.len() > 1 {
            if v.contains(k) {
                packages.push(k.to_string());
            }
            let masked = get_masked_name(k);
            packages.push(masked);
        }
        else {
            let key = k.to_string();
            let first_value = v[0].to_string();

            if key == first_value {
                packages.push(key);
            }
            else {
                let masked = get_masked_name(k);
                packages.push(masked);
            }
        }
    }

    packages.sort();
    packages
}

fn get_masked_name(k: &str) -> String {
    let mut masked = String::with_capacity(k.len() + 3);
    masked.push_str(k);
    masked.push_str(".*");
    masked
}

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