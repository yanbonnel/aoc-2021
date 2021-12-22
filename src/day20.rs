use std::cmp::max;
use std::collections::{HashMap, HashSet};


#[derive(Debug, Clone)]
struct Map {
    map: Vec<String>,
}


fn pow2(pow: usize) -> usize {
    let mut result: usize = 1;
    for _ in 0..pow {
        result = result * 2;
    }
    result
}


fn binary_to_dec(data: &String) -> usize {
    let mut binary = data.chars().map(|car| if car == '#' { 1 } else { 0 } as usize).collect::<Vec<_>>();

    binary.reverse();

    let result = binary.into_iter().enumerate().map(|(index, data)|
        data * pow2(index)
    ).sum();

    result
}

impl Map {
    fn exisiting_car(&self, x: i32, y: i32) -> Option<char> {
        if x >= 0 && x < self.map.get(0).unwrap().len() as i32
            && y >= 0 && y < self.map.len() as i32
        {
            Some(self.map.get(y as usize).unwrap().chars().nth(x as usize).unwrap())
        } else {
            None
        }
    }

    fn next_car(&self, x: usize, y: usize, enhancement: &String) -> char {
        let x = x as i32;
        let y = y as i32;
        let input: String = (y..=(y + 2)).flat_map(|search_y|
            (x..=(x + 2)).map(move |search_x|
                self.exisiting_car(search_x - 1, search_y - 1).unwrap_or(self.exisiting_car(0, 0).unwrap())
            )
        ).collect();


        let result = enhancement.chars().nth(binary_to_dec(&input)).unwrap();

        result
    }

    fn next(&self, enhancement: &String) -> Self {
        let len_y = self.map.len();
        let len_x = self.map.get(0).unwrap().len();
        Map {
            map: (0..len_y).map(|y| {
                (0..len_x).map(|x| {
                    self.next_car(x, y, enhancement)
                }).collect()
            }).collect()
        }
    }

    fn prepare_for_next(&self) -> Self {
        let len_y: i32 = self.map.len() as i32;
        let len_x: i32 = self.map.get(0).unwrap().len() as i32;
        Map {
            map: (-1..(len_y + 1)).map(|y|
                (-1..(len_x + 1)).map(|x| {
                    self.exisiting_car(x, y).unwrap_or('.')
                }).collect()
            ).collect()
        }
    }

    fn print(&self) {
        self.map.iter().for_each(|line| {
            println!("{}", line)
        })
    }

    fn count(&self) -> usize {
        self.map.iter().map(|line|
            line.chars().filter(|car| *car == '#')
                .count()
        ).sum()
    }
}


pub fn execute(input: &str) {
    let input: Vec<_> = input.split("\n\n")
        .collect();

    let enhancement = input[0].replace("\n", "");

    let input: Vec<String> = input[1].split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string()).collect();

    let map = Map {
        map: input
    };

    {

        // Step 1
        let map_step1 = map.clone();
        let map_step1 = map_step1.prepare_for_next().prepare_for_next().prepare_for_next().prepare_for_next();
        let map_step1 = map_step1.next(&enhancement).next(&enhancement);

        map_step1.print();
        println!("Result step_1 : {}", map_step1.count())

    }

    {
        // step 2
        let map = map.clone();

        let map = (0..60).fold(map, |map, idx| {
            println!("Prepare {}/50", idx);
            map.prepare_for_next()
        });

        let map = (0..50).fold(map, |map, idx| {
            println!("{}/50", idx);
            map.next(&enhancement)
        });

        map.print();
        println!("Step 2 result : {}", map.count());
    }
}