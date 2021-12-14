use std::collections::{HashSet, HashMap};


pub fn execute(input: &str) {
    let values: Vec<_> = input.split("\n").map(|v| v.to_string())
        .filter(|line| !line.is_empty())
        .collect();

    let start_pattern = values[0].clone().chars().collect::<Vec<_>>();

    let first_car = start_pattern[0];

    let rules = values.into_iter().skip(1).map(|line| {
        let key = line.split(" -> ").nth(0).unwrap().to_string();
        let key = (key.chars().nth(0).unwrap(), key.chars().nth(1).unwrap());
        let value = line.split(" -> ").nth(1).unwrap().chars().nth(0).unwrap();
        (key, value)
    }).collect::<HashMap<_, _>>();

    let pairs: Vec<(char, char)> = start_pattern.clone().windows(2).map(|cars|
        (cars[0], cars[1])
    ).collect();
    let mut pattern: HashMap<(char, char), u128> = pairs.into_iter().enumerate().map(|(idx, (c1, c2))| {
        ((c1, c2), 1 as u128)
    }).fold(HashMap::new(), |mut acc, (key, count)| {
        acc.insert(key, acc.get(&key).cloned().unwrap_or(0) + count);
        acc
    });

    println!("{:?}", pattern);


    for idx in 0..40 {
        if idx == 10 {
            let step1_result = calculate_result(&pattern, first_car);

            println!("Step 1 result {}", step1_result);
        }
        pattern = iterate(&pattern, &rules);
    }
    let step2_result = calculate_result(&pattern, first_car);

    println!("Step 2 result {}", step2_result);

}

fn iterate(pattern: &HashMap<(char, char), u128>, rules: &HashMap<(char, char), char>) -> HashMap<(char, char), u128> {
    let mut new_pattern = HashMap::new();
    for ((c1, c2), count) in pattern {
        if let Some(insert) = rules.get(&(*c1, *c2)) {
            let key1 = (*c1, *insert);
            let key2 = (*insert, *c2);
            new_pattern.insert(key1, new_pattern.get(&key1).cloned().unwrap_or(0) + count);
            new_pattern.insert(key2, new_pattern.get(&key2).cloned().unwrap_or(0) + count);
        }
    }
    new_pattern
}

fn calculate_result(pattern: &HashMap<(char, char), u128>, first_car: char) -> u128 {
    let mut init_map = HashMap::new();
    init_map.insert(first_car, 1);
    let mut result = pattern.iter().map(|((c1, c2), count)| {
        (*c2, *count)
    })
        .fold(init_map, |mut acc: HashMap<char, u128>, (car, count)| {
            acc.insert(car, acc.get(&car).cloned().unwrap_or(0) + count);
            acc
        });

    let most_common = result.values().max().unwrap();
    let less_common = result.values().min().unwrap();

    println!("most_common : {}", most_common);
    println!("less_common : {}", less_common);

    let step1_result = (*most_common - *less_common);
    step1_result
}
