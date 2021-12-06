
struct Counter {
    pub count_0: usize,
    pub count_1: usize
}
pub fn pow2(pow: usize) -> u32 {
    (0..pow).fold(1, |acc, item|
        acc * 2
    )
}

pub fn bits_to_dec(line: String) -> u32 {
    line.chars().rev().enumerate().fold(0, |acc, (idx, car)|
        if car == '1' {
            acc + pow2(idx)
        } else {
            acc
        }
    )
}

pub fn execute(input: &str) {

    let values: Vec<_> = input.split("\n").map(|v| v.to_string()).collect();

    let length = values[0].len();

    let mut result: Vec<Counter> = count_cars(&values, length);

    result.reverse();

    let (gamma, epsilon) = result.into_iter().enumerate().fold((0, 0), |(gamma, epsilon), (idx, counter)| {
        if counter.count_1 >= counter.count_0 {
            (gamma + pow2(idx), epsilon)
        } else {
            (gamma, epsilon + pow2(idx))
        }
    });

    println!("Gamma : {}, Epsilon : {}", gamma, epsilon);
    println!("Step 1 result : {}", gamma * epsilon);

    let (oxy_bits, co2_bits) = (0..length).fold((values.clone(), values.clone()), |(oxygen_values, co2_values), idx| {
        (if oxygen_values.len() <= 1 {
            oxygen_values.clone()
        } else {
            let count_1 = oxygen_values.iter().filter(|line| line.chars().nth(idx).unwrap() == '1').count();
            let count_0 = oxygen_values.len() - count_1;
            let most_common_car = if count_1 >= count_0 { '1' } else { '0' };
            oxygen_values.clone().into_iter().filter(|line| line.chars().nth(idx).unwrap() == most_common_car).collect()
        },
        if co2_values.len() <= 1 {
            co2_values.clone()
        } else {
            let count_1 = co2_values.iter().filter(|line| line.chars().nth(idx).unwrap() == '1').count();
            let count_0 = co2_values.len() - count_1;
            let less_common_car = if count_1 < count_0 { '1' } else { '0' };
            co2_values.clone().into_iter().filter(|line| line.chars().nth(idx).unwrap() == less_common_car).collect()
        })
    });

    let oxy = bits_to_dec(oxy_bits[0].clone());
    let co2 = bits_to_dec(co2_bits[0].clone());


    println!("Oxygen : {}, CO2 : {}", oxy, co2);
    println!("Step 2 result : {}", oxy * co2);



}

fn count_cars(values: &Vec<String>, length: usize) -> Vec<Counter> {
    values.iter().fold((0..length).map(|_| Counter { count_0: 0, count_1: 0 }).collect(), |acc, item| {
        let cars: Vec<_> = item.chars().collect();
        acc.into_iter().enumerate().map(|(idx, counter)|
            match cars[idx] {
                '0' => Counter {
                    count_0: counter.count_0 + 1,
                    count_1: counter.count_1
                },
                _ => Counter {
                    count_0: counter.count_0,
                    count_1: counter.count_1 + 1
                },
            }
        ).collect()
    })
}