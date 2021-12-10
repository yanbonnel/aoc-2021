
fn is_open(car: char) -> bool {
    match car {
        '(' | '{' | '[' | '<' => true,
        _ => false
    }
}

fn corresponding_close(car: char) -> char {
    match car {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => panic!()
    }
}

fn score(wrong_car: char) -> u128 {
    match wrong_car {
        ')' => 3,
        '}' => 1197,
        ']' => 57,
        '>' => 25137,
        _ => panic!()
    }
}

fn score_2(car: char) -> u128 {
    match car {
        ')' => 1,
        '}' => 3,
        ']' => 2,
        '>' => 4,
        _ => panic!()
    }
}

pub fn analyse(line: String) -> (bool, u128) {
    let mut open: Vec<char> = vec![];

    for car in line.chars() {
        if is_open(car) {
            open.push(car);
        } else {
            let last_open = open.remove(open.len() - 1);
            let expected_close = corresponding_close(last_open);
            if expected_close != car {
                let score = score(car);
                return (true, score)
            }
        }
    }

    open.reverse();
    let score: u128 = open.into_iter().map(|car| corresponding_close(car))
        .fold(0, |acc, car| {
            acc * 5 + score_2(car)
        });

    (false, score)
}

pub fn execute(input: &str) {
    let values: Vec<String> = input.split("\n").map(|v| v.to_string())
        .collect();

    let step1_result: u128 = values.clone().into_iter().map(|line| analyse(line))
        .filter(|(wrong, score)| *wrong)
        .map(|(_, score)| score)
        .sum();

    println!("Step 1 result : {}", step1_result);



    let mut values: Vec<u128> = values.into_iter().map(|line| analyse(line))
        .filter(|(wrong, score)| !*wrong)
        .map(|(_, score)| score).collect();

    values.sort();

    let step2_result = values.get((values.len() - 1) / 2).unwrap();

    println!("Step 2 result : {}", step2_result)



}
