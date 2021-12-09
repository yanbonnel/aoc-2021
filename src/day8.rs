use std::collections::{HashMap, HashSet};
use crate::day3::pow2;

pub fn initial_mapping() -> HashMap<char, Vec<char>> {
    ('a'..='g').map(|car|
        (car, ('a'..='g').collect())
    ).collect()
}

pub fn expend(cars: Vec<HashMap<char, char>>) -> Vec<HashMap<char, char>> {
    let result: Vec<HashMap<char, char>> = cars.into_iter().flat_map(|cars| {
        let next_car =
            std::char::from_u32('a' as u32 + cars.len() as u32).unwrap();
        let already_known_cars = cars.values().cloned().collect::<HashSet<_>>();
        let result = ('a'..='g').filter(move |car| !already_known_cars.contains(car)).map(move |car| {
            let mut next_cars = cars.clone();
            next_cars.insert(next_car, car);
            next_cars
        }).collect::<Vec<_>>();

        result
    }).collect();

    result
}

pub fn all_possible_mapping() -> Vec<HashMap<char, char>> {
    let mut result = ('a'..='g').map(|car|
        std::iter::once(('a', car)).collect()
    ).collect();
    for _ in 'b'..='g' {
        result = expend(result);
    }
    result
}

fn works(input: &str, mapping: &HashMap<char, char>) -> bool {

    let digits: Vec<(u8, &str)> = vec![
        (0, "abcefg"),
        (1, "cf"),
        (2, "acdeg"),
        (3, "acdfg"),
        (4, "bcdf"),
        (5, "abdfg"),
        (6, "abdefg"),
        (7, "acf"),
        (8, "abcdefg"),
        (9, "abcdfg")
    ];

    let possible_values: HashSet<&str> = digits.iter().map(|(_, cars)| cars.clone()).collect();
    let mut translated_input = input.chars().map(|car| *mapping.get(&car).unwrap())
        .collect::<Vec<_>>();

    translated_input.sort();

    let translated_input = translated_input.into_iter().collect::<String>();
    possible_values.contains(translated_input.as_str())
}

pub fn execute(input: &str) {

    let values: Vec<_> = input.split("\n").map(|v| v.to_string())
        .collect();

    //1, 4, 7, 8
    let step_1_result:u64 = values.iter().map(|line| {
        let input_output = line.split("|").collect::<Vec<_>>();
        let inputs = input_output[0].trim();
        let outputs = input_output[1].trim().split(" ").collect::<Vec<_>>();
        let count = outputs.into_iter().filter(|digit| digit.len() == 2 || digit.len() == 4 || digit.len() == 3 || digit.len() == 7)
            .count() as u64;

        count
    }).sum();

    println!("Step 1 result : {}", step_1_result);


    let result_step2: u128 = values.iter().map(|line| {
        let input_output = line.split("|").collect::<Vec<_>>();
        let inputs = input_output[0].trim().split(" ").collect::<Vec<_>>();
        let outputs = input_output[1].trim().split(" ").collect::<Vec<_>>();

        let all_possible_mappings = all_possible_mapping();

        //println!("All possible mapping : {:?}", all_possible_mappings);

        let result = all_possible_mappings.into_iter().filter(|possible_mapping| {
            inputs.iter().all(|input| works(input, possible_mapping))
        }).collect::<Vec<_>>();

        if result.len() != 1 {
            panic!("WTF");
        }
        let mapping = result[0].clone();

        let digits = vec![
            (0, "abcefg"),
            (1, "cf"),
            (2, "acdeg"),
            (3, "acdfg"),
            (4, "bcdf"),
            (5, "abdfg"),
            (6, "abdefg"),
            (7, "acf"),
            (8, "abcdefg"),
            (9, "abcdfg")
        ];

        let translator = digits.into_iter().map(|(digit, cars)|  {
            let mut translated_cars = cars.chars().map(|car| {
                mapping.iter().filter_map(|(source, target)|
                    if *target == car {
                        Some(*source)
                    } else {
                        None
                    }
                ).next().unwrap()
            })
                 .collect::<Vec<char>>();
            translated_cars.sort();

            (translated_cars.into_iter().collect(), digit)
        }).collect::<HashMap<String, i32>>();


        let outputs: Vec<String> = outputs.into_iter().map(|output| {
            let mut result = output.chars().collect::<Vec<char>>();
            result.sort();
            result.into_iter().collect()
        }).collect();

        // cdafg

        let result: i32 = outputs.into_iter().map(|output|
            translator.get(&output).unwrap()
        ).rev().enumerate().map(|(idx, digit)| {
            (*digit) * pow10(idx)
        }).sum();

        println!("Result : {}", result);

        result as u128

    }).sum();

    println!("Step 2 result : {}", result_step2)

}
pub fn pow10(pow: usize) -> i32 {
    (0..pow).fold(1, |acc, item|
        acc * 10
    )
}
