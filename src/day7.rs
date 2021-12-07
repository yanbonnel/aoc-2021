
struct Crab {
    position: i32
}

impl Crab {
    fn fuel_needed(&self, target: i32) -> i32 {
        (self.position - target).abs()
    }
    fn fuel_needed_2(&self, target: i32) -> i32 {
        if target == self.position {
            return 0
        }
        (1..=(self.position - target).abs()).sum()
    }
}

pub fn execute(input: &str) {
    let values: Vec<_> = input.split(",").map(|v| v.to_string())
        .map(|v| Crab {
            position: v.parse().unwrap()
        })
        .collect();

    let min = values.iter().map(|crab| crab.position).min().unwrap();
    let max = values.iter().map(|crab| crab.position).max().unwrap();

    let step_1_result: i32 = (min ..= max).map(|target|
        values.iter().map(|crab| crab.fuel_needed(target)).sum()
    ).min().unwrap();

    println!("step 1 result : {}", step_1_result);

    let step_2_result: i32 = (min..=max).map(|target|
        values.iter().map(|crab| crab.fuel_needed_2(target)).sum()
    ).min().unwrap();

    println!("step 2 result : {}", step_2_result);


}
