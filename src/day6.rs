use std::collections::{HashSet, HashMap};

#[derive(Clone)]
struct Fish {
    life: i32,
    count: u64,
}

impl Fish {
    fn next(&self) -> Vec<Fish> {
        if self.life == 0 {
            vec![Fish {
                life: 6,
                count: self.count
            }, Fish {
                life: 8,
                count: self.count
            }]
        } else {
            vec![Fish {
                life: self.life - 1,
                count: self.count
            }]
        }

    }
}

fn reduce_fishes(fishes: Vec<Fish>) -> Vec<Fish> {
    fishes.into_iter().fold(HashMap::<i32, Fish>::new(), |mut acc, fish| {
        let actual_fish = acc.get(&fish.life).map(|fish| fish.clone()).unwrap_or(Fish {
            life: fish.life,
            count: 0
        });
        acc.insert(fish.life, Fish {
            life: fish.life,
            count: actual_fish.count + fish.count
        });
        acc
    } ).into_iter()
        .map(|(_, fish)| fish)
        .collect()
}

fn count_fishes(fishes: &Vec<Fish>) -> u64 {
    fishes.iter().fold(0, |acc, fish| acc + fish.count)
}



pub fn execute(input: &str) {
    let values: Vec<_> = input.split(",").map(|v| v.to_string())
        .map(|counter| Fish { life: counter.parse().unwrap(), count: 1})
        .collect();

    let mut fishes = values.clone();

    for idx in 0..256 {
        fishes = reduce_fishes(fishes.iter().flat_map(|fish| fish.next()).collect());

        if idx == 79 {
            println!("Step 1 : {}", count_fishes(&fishes));
        }
    }

    println!("Step 2 : {}", count_fishes(&fishes));


}
