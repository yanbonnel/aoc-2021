use std::collections::{HashSet, HashMap};

struct Path {
    position: (usize, usize),
    risk: u32,
}

impl Path {
    fn next(self, risks: &Vec<Vec<u32>>) -> Vec<Path> {
        let mut result = vec![];
        let max_x = risks[0].len() - 1;
        let max_y = risks.len() - 1;
        if self.position == (
            max_x, max_y
        ) {
            return vec![self];
        }

        let (x, y) = self.position;

        // Left
        if x > 0 {
            result.push(Path {
                position: (x - 1, y),
                risk: self.risk + risks[y][x - 1],
            })
        }
        // Up
        if y > 0 {
            result.push(Path {
                position: (x, y - 1),
                risk: self.risk + risks[y - 1][x],
            })
        }
        // Right
        if x < max_x {
            result.push(Path {
                position: (x + 1, y),
                risk: self.risk + risks[y][x + 1],
            })
        }
        // Down

        if y < max_y {
            result.push(Path {
                position: (x, y + 1),
                risk: self.risk + risks[y + 1][x],
            })
        }

        result
    }
}

fn new_risk(new_risk: u32) -> u32 {
    let mut new_risk = new_risk;
    while new_risk > 9 {
        new_risk = new_risk - 9
    }

    new_risk
}

pub fn execute(input: &str) {
    let risks: Vec<Vec<_>> = input.split("\n").map(|v| v.to_string())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|car| car.to_digit(10).unwrap()).collect())
        .collect();

    let mut positions = vec![Path {
        position: (0, 0),
        risk: 0,
    }];

    for _ in 0..=(risks.len() * 3) {
        positions = positions.into_iter().flat_map(|path| path.next(&risks)).fold(HashMap::new(), |mut acc: HashMap<(usize, usize), u32>, path| {
            if !acc.contains_key(&path.position) || *acc.get(&path.position).unwrap() > path.risk {
                acc.insert(path.position, path.risk);
            }
            acc
        }).into_iter().map(|(position, risk)|
            Path {
                position,
                risk,
            }
        ).collect()
    }

    let step1_result: u32 = positions.into_iter()
        .filter(|path|
            path.position == (risks[0].len() - 1, risks.len() - 1)
        )
        .map(|path| path.risk).min().unwrap();

    println!("Step 1 result : {}", step1_result);

    let mut risks_step2 = risks.clone();
    for y_time in 1..5 {
        risks.iter().map(|line|
            line.iter().map(|risk| new_risk(*risk + y_time)).collect()
        ).for_each(|line: Vec<u32>|
            risks_step2.push(line)
        )
    }
    risks_step2.iter_mut().for_each(|line| {
        let original_line = line.clone();
        for x_time in 1..5 {
            original_line.iter().map(|risk| new_risk(*risk + x_time)).for_each(|new_risk|
                line.push(new_risk)
            )
        }
    });

    let mut positions = vec![Path {
        position: (0, 0),
        risk: 0,
    }];
    let risks = risks_step2;


    for idx in 0..=(risks.len() * 3) {
        positions = positions.into_iter().flat_map(|path| path.next(&risks)).fold(HashMap::new(), |mut acc: HashMap<(usize, usize), u32>, path| {
            if !acc.contains_key(&path.position) || *acc.get(&path.position).unwrap() > path.risk {
                acc.insert(path.position, path.risk);
            }
            acc
        }).into_iter().map(|(position, risk)|
            Path {
                position,
                risk,
            }
        ).collect();

        println!("{} / {} : path_explored : {}", idx, risks.len() * 3, positions.len());
    }


    let step2_result: u32 = positions.into_iter()
        .filter(|path|
            path.position == (risks[0].len() - 1, risks.len() - 1)
        )
        .map(|path| path.risk).min().unwrap();

    println!("Step 2 result : {}", step2_result);
}
