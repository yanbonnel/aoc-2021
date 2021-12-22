use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};


#[derive(Debug, Clone)]
struct Step {
    on: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

#[derive(Debug, Clone)]
struct Cube {
    min: (i32, i32, i32),
    max: (i32, i32, i32),
}


impl Cube {
    fn size(&self) -> u128 {
        let x_size = (self.max.0 - self.min.0 + 1) as u128;
        let y_size = (self.max.1 - self.min.1 + 1) as u128;
        let z_size = (self.max.2 - self.min.2 + 1) as u128;
        x_size * y_size * z_size
    }

    fn cover(&self, other: &Cube) -> bool {
        let has_x_in = self.min.0 <= other.max.0 && self.max.0 >= other.min.0;
        let has_y_in = self.min.1 <= other.max.1 && self.max.1 >= other.min.1;
        let has_z_in = self.min.2 <= other.max.2 && self.max.2 >= other.min.2;
        has_x_in && has_y_in && has_z_in
    }

    fn is_realist(&self) -> bool {
        self.min.0 <= self.max.0 && self.min.1 <= self.max.1 && self.min.2 <= self.max.2
    }

    fn minus(self, other: &Cube) -> Vec<Cube> {
        if !self.cover(other) {
            vec![self]
        } else {
            vec![
                // Up
                Cube {
                    min: self.min,
                    max: (self.max.0, other.min.1 - 1, self.max.2),
                },
                // Down
                Cube {
                    min: (self.min.0, other.max.1 + 1, self.min.2),
                    max: self.max,
                },
                // Left (min_x) (10, 11, 10), (10, 12, 12)
                Cube {
                    min: (self.min.0, max(other.min.1 - 1, self.min.1 - 1) + 1, self.min.2),
                    max: (other.min.0 - 1, min(other.max.1, self.max.1), self.max.2),
                },
                // Right (max_x)
                Cube {
                    min: (other.max.0 + 1, max(other.min.1 - 1, self.min.1 - 1) + 1, self.min.2),
                    max: (self.max.0, min(other.max.1, self.max.1), self.max.2),
                },
                // min_z (11, 11, 10), (11, 11, 10)
                Cube {
                    min: (max(other.min.0 - 1, self.min.0 - 1) + 1, max(other.min.1 - 1, self.min.1 - 1) + 1, self.min.2),
                    max: (min(other.max.0, self.max.0), min(other.max.1, self.max.1), other.min.2 - 1),
                },
                // max_z
                Cube {
                    min: (max(other.min.0 - 1, self.min.0 - 1) + 1, max(other.min.1 - 1, self.min.1 - 1) + 1, other.max.2 + 1),
                    max: (min(other.max.0, self.max.0), min(other.max.1, self.max.1), self.max.2),
                },
            ].into_iter().filter(|cube| cube.is_realist()).collect()
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    cubes: Vec<Cube>,
}

impl Map {
    fn add_cube(self, cube_on: Cube) -> Self {
        let mut new_cubes: Vec<Cube> = self.cubes.into_iter().flat_map(|cube| cube.minus(&cube_on)).collect();
        new_cubes.push(cube_on);
        Map {
            cubes: new_cubes
        }
    }

    fn remove_cube(self, cube_off: Cube) -> Self {
        Map {
            cubes: self.cubes.into_iter().flat_map(|cube| cube.minus(&cube_off)).collect()
        }
    }

    fn size(&self) -> u128 {
        self.cubes.iter().map(|cube| cube.size()).sum()
    }
}

impl Step {
    fn new(line: &str) -> Self {
        let on = line.starts_with("on");
        let coords: Vec<_> = line.split(" ").nth(1)
            .unwrap()
            .split(",")
            .map(|coord| coord.split("=").nth(1).unwrap())
            .map(|coord| {
                let coord: Vec<i32> = coord.split("..")
                    .map(|d| d.parse().unwrap())
                    .collect();
                (coord[0], coord[1])
            }).collect();
        Step {
            on,
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }

    fn is_step_1_valid(&self) -> bool {
        is_pos_valid(self.x.0) && is_pos_valid(self.x.1)
            && is_pos_valid(self.y.0) && is_pos_valid(self.y.1)
            && is_pos_valid(self.z.0) && is_pos_valid(self.z.1)
    }

    fn all_pos(&self) -> HashSet<(i32, i32, i32)> {
        (self.x.0..=self.x.1).flat_map(|x|
            (self.y.0..=self.y.1).flat_map(move |y|
                (self.z.0..=self.z.1).map(move |z|
                    (x, y, z)
                )
            )
        ).collect()
    }

    fn cube(&self) -> Cube {
        Cube {
            min: (self.x.0, self.y.0, self.z.0),
            max: (self.x.1, self.y.1, self.z.1),
        }
    }
}

fn is_pos_valid(pos: i32) -> bool {
    pos >= -50 && pos <= 50
}


pub fn execute(input: &str) {
    let input: Vec<_> = input.split("\n")
        .map(|line| Step::new(line))
        .collect();

    let first = AtomicBool::new(true);

    let result = input.iter()
        .fold(Map {
            cubes: vec![]
        }, |acc, step| {
            if !step.is_step_1_valid() && first.fetch_and(false, Ordering::Acquire) {
                println!("### Step 1 result : {}", acc.size())
            }
            if step.on {
                acc.add_cube(step.cube())
            } else {
                acc.remove_cube(step.cube())
            }
        });


    println!("Actual size : {}", result.size());
}