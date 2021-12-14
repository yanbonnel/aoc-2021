use std::collections::{HashSet, HashMap};


pub fn execute(input: &str) {
    let values: Vec<_> = input.split("\n").map(|v| v.to_string())
        .filter(|line| !line.is_empty())
        .collect();

    let start_pattern = values[0].clone().chars().collect::<Vec<_>>();

    let rules = values.into_iter().skip(1).map(|line| {
        let key = line.split(" -> ").nth(0).unwrap().to_string();
        let key = (key.chars().nth(0).unwrap(), key.chars().nth(1).unwrap());
        let value = line.split(" -> ").nth(1).unwrap().chars().nth(0).unwrap();
        (key, value)
    }).collect::<HashMap<_, _>>();

    let mut pattern = start_pattern.clone();

    for _ in 0..10 {

        pattern = iterate(&pattern, &rules);
    }

    let step1_result = calculate_result(&pattern);

    println!("Step 1 result {}", step1_result);



}

fn iterate(pattern: &Vec<char>, rules: &HashMap<(char, char), char>) -> Vec<char> {
    let mut next_pattern = vec![pattern[0]];
    for idx in 1..pattern.len() {
        let last_key = (pattern[idx - 1], pattern[idx]);
        if let Some(insert) = rules.get(&last_key) {
            next_pattern.push(*insert)
        }
        next_pattern.push(pattern[idx]);
    }
    next_pattern
}

fn calculate_result(pattern: &Vec<char>) -> u128 {
    let result = pattern.iter().fold(HashMap::new(), |mut acc: HashMap<char, u128>, car| {
        if !acc.contains_key(&car) {
            acc.insert(*car, 0);
        };
        acc.insert(*car, acc.get(&car).unwrap() + 1);
        acc
    });

    let most_common = result.values().max().unwrap();
    let less_common = result.values().min().unwrap();

    let step1_result = *most_common - *less_common;
    step1_result
}
