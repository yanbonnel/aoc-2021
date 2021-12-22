use std::cmp::max;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

impl Beacon {
    fn new(input: &str) -> Self {
        let result: Vec<i32> = input.split(",").map(|d|
            d.parse::<i32>().unwrap()
        ).collect();

        let x = result[0];
        let y = result[1];
        let z = result[2];

        Beacon {
            x,
            y,
            z,
        }
    }

    fn dist(&self, other: &Self) -> (i32, i32, i32) {
        (other.x - self.x, other.y - self.y, other.z - self.z)
    }

    fn translate(&self, dist: (i32, i32, i32)) -> Self {
        Self {
            x: self.x - dist.0,
            y: self.y - dist.1,
            z: self.z - dist.2,
        }
    }

    fn rotate(&self, id: i32) -> Self {
        match id {
            0 => Self {
                x: self.x,
                y: self.y,
                z: self.z,
            },
            1 => Self {
                x: self.x,
                y: -self.y,
                z: -self.z,
            },
            2 => Self {
                x: self.y,
                y: -self.z,
                z: -self.x,
            },
            3 => Self {
                x: self.z,
                y: -self.y,
                z: self.x,
            },
            4 => Self {
                x: -self.z,
                y: -self.y,
                z: -self.x,
            },
            5 => Self {
                x: self.y,
                y: self.z,
                z: self.x,
            },
            6 => Self {
                x: self.z,
                y: -self.x,
                z: -self.y,
            },
            7 => Self {
                x: self.x,
                y: -self.z,
                z: self.y,
            },
            8 => Self {
                x: -self.x,
                y: -self.z,
                z: -self.y,
            },
            9 => Self {
                x: -self.y,
                y: -self.x,
                z: -self.z,
            },
            10 => Self {
                x: -self.y,
                y: -self.z,
                z: self.x,
            },
            11 => Self {
                x: -self.x,
                y: -self.y,
                z: self.z,
            },
            12 => Self {
                x: -self.x,
                y: self.z,
                z: self.y,
            },
            13 => Self {
                x: -self.z,
                y: self.y,
                z: self.x,
            },
            14 => Self {
                x: -self.x,
                y: self.y,
                z: -self.z,
            },
            15 => Self {
                x: -self.y,
                y: self.x,
                z: self.z,
            },
            16 => Self {
                x: self.y,
                y: -self.x,
                z: self.z,
            },
            17 => Self {
                x: self.z,
                y: self.y,
                z: -self.x,
            },
            18 => Self {
                x: -self.y,
                y: self.z,
                z: -self.x,
            },
            19 => Self {
                x: -self.z,
                y: -self.x,
                z: self.y,
            },
            20 => Self {
                x: self.y,
                y: self.x,
                z: -self.z,
            },
            21 => Self {
                x: self.z,
                y: self.x,
                z: self.y,
            },
            22 => Self {
                x: -self.z,
                y: self.x,
                z: -self.y,
            },
            23 => Self {
                x: self.x,
                y: self.z,
                z: -self.y,
            },
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn new(input: &str) -> Self {
        Scanner {
            beacons: input.split("\n").skip(1).map(|input| Beacon::new(input)).collect()
        }
    }

    fn has_common_beacon(&self, other: &Scanner, rotate: i32) -> Option<(i32, i32, i32)> {
        let distances = self.beacons.iter().flat_map(|a|
            other.beacons.iter().map(move |b| {
                let b_rotated = b.rotate(rotate);
                a.dist(&b_rotated)
            })
        ).fold(HashMap::new(), |mut acc: HashMap<(i32, i32, i32), usize>, item| {
            acc.insert(item, acc.get(&item).cloned().unwrap_or(0) + 1);
            acc
        });
        distances.into_iter().find(|(item, counter)| *counter >= 12)
            .map(|(item, _)| item)
    }

    fn adjust(&self, rotate: i32, dist: (i32, i32, i32)) -> Self {
        Self {
            beacons: self.beacons.iter().map(|beacon|
                beacon.rotate(rotate).translate(dist)
            ).collect()
        }
    }
}


pub fn execute(input: &str) {
    let mut scanners: Vec<_> = input.split("\n\n")
        .map(|input| Scanner::new(input))
        .collect();

    let mut explored_scanners: Vec<(Scanner, (i32, i32, i32))> = vec![];

    explored_scanners.push((scanners.remove(0), (0, 0, 0)));

    while !scanners.is_empty() {
        let (explored_scanner_id, scanner_id, rotate, dist) = (0..explored_scanners.len())
            .flat_map(|explored_scanner_id|
                (0..scanners.len()).flat_map(move |scanner_id|
                    (0..24).map(move |rotate|
                        (explored_scanner_id, scanner_id, rotate)
                    )
                )
            ).filter_map(|(explored_scanner_id, scanner_id, rotate)| {
            let (explored_scanner, _) = explored_scanners.get(explored_scanner_id).unwrap();
            let scanner = scanners.get(scanner_id).unwrap();
            explored_scanner.has_common_beacon(scanner, rotate).map(|dist|
                (explored_scanner_id, scanner_id, rotate, dist)
            )
        }).next().unwrap();
        let scanner_pos = (
            -dist.0,
            -dist.1,
            -dist.2,
            );
        explored_scanners.push((scanners.remove(scanner_id)
            .adjust(rotate, dist), scanner_pos));
        println!("{}/{}", explored_scanners.len(), explored_scanners.len() + scanners.len());
    }

    let unique_beacons = explored_scanners.clone().into_iter().flat_map(|(scanner, _)| scanner.beacons)
        .collect::<HashSet<Beacon>>();

    println!("Step 1 result : {}", unique_beacons.len());

    let max_dist = explored_scanners.iter().flat_map(|(_, (x_a, y_a, z_a))|
        explored_scanners.iter().map(move |(_, (x_b, y_b, z_b))|
            (*x_a - *x_b).abs() + (*y_a - *y_b).abs() + (*z_a - *z_b).abs()
        )
    ).max().unwrap();

    println!("Step 2 result {}", max_dist);
}