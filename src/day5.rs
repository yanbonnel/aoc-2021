use std::collections::HashMap;

struct Wind {
    start: (i32, i32),
    end: (i32, i32),
}

impl Wind {
    fn new(line: String) -> Self {
        let positions = line.split(" -> ").collect::<Vec<_>>();
        Wind {
            start: (
                positions[0].split(",").next().unwrap().parse().unwrap(),
                positions[0].split(",").nth(1).unwrap().parse().unwrap(),
            ),
            end: (
                positions[1].split(",").next().unwrap().parse().unwrap(),
                positions[1].split(",").nth(1).unwrap().parse().unwrap(),
            ),
        }
    }

    fn is_only_hori_or_vert(&self) -> bool {
        let (x1, y1) = self.start;
        let (x2, y2) = self.end;
        x1 == x2 || y1 == y2
    }

    fn covert_points(&self) -> Vec<(i32, i32)> {
        let (x1, y1) = self.start;
        let (x2, y2) = self.end;
        let inc_x = if x2 - x1 < 0 {
            -1
        } else if x2 - x1 > 0 {
            1
        } else {
            0
        };
        let inc_y = if y2 - y1 < 0 {
            -1
        } else if y2 - y1 > 0 {
            1
        } else {
            0
        };
        let mut current_x = x1;
        let mut current_y = y1;
        let mut result = vec![(current_x, current_y)];
        while current_x != x2 || current_y != y2 {
            current_x = current_x + inc_x;
            current_y = current_y + inc_y;
            result.push((current_x, current_y));
        }

        result
    }
}

pub fn execute(input: &str) {
    let values: Vec<_> = input.split("\n").map(|v| v.to_string())
        .map(|line| Wind::new(line))
        .collect();

    let map: HashMap<(i32, i32), i32> = values.iter().filter(|wind| wind.is_only_hori_or_vert())
        .flat_map(|wind| wind.covert_points())
        .fold(HashMap::new(), |mut acc, item| {
            let current_counter = acc.get(&item).map(|i| *i).unwrap_or(0);
            acc.insert(item, current_counter + 1);
            acc
        });

    let step_1_result = map.values().map(|i| *i).filter(|i| *i >= 2).count();

    println!("Step1 result {}", step_1_result);

    let map: HashMap<(i32, i32), i32> = values.iter()
        .flat_map(|wind| wind.covert_points())
        .fold(HashMap::new(), |mut acc, item| {
            let current_counter = acc.get(&item).map(|i| *i).unwrap_or(0);
            acc.insert(item, current_counter + 1);
            acc
        });

    let step_2_result = map.values().map(|i| *i).filter(|i| *i >= 2).count();

    println!("Step2 result {}", step_2_result);
}
