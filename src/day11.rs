use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Octopus {
    life: u8,
    count_flash: u128,
    must_flash: bool
}

impl Octopus {
    fn inc(&mut self) {
        if self.life < 10 {
            self.life = self.life + 1;
            if self.life == 10 {
                self.must_flash = true
            }
        }
    }

    fn count_flash(&mut self) -> u8 {
        if self.life > 9 {
            self.count_flash = self.count_flash + 1;
            self.life = 0;
            1
        } else {
            0
        }
    }
}

fn flashes(octupuses: &mut Vec<Vec<Octopus>>) -> bool {

    let flashes = octupuses.iter().enumerate().flat_map(|(y, line)| {
        let y = y + 0;
        line.iter().enumerate().filter(|(x, octopus)| octopus.must_flash).map(move |(x, _)| (x as i8, y as i8))
    }).flat_map(|(x, y)|
        vec![(x - 1, y),
             (x + 1, y),
             (x, y- 1),
             (x, y + 1),
             (x - 1, y -1),
             (x + 1, y - 1),
             (x - 1, y + 1),
             (x + 1, y + 1),
        ]
    ).filter(|(x, y)|
        *x >= 0 && *x < 10 && *y >= 0 && *y < 10
    ).map(|(x, y)|
        (x as usize, y as usize)
    )
        .collect::<Vec<(usize, usize)>>();


    octupuses.iter_mut().for_each(|line| line.iter_mut().for_each(|octopus| octopus.must_flash = false));


    let has_flash = !flashes.is_empty();

    for (x, y) in flashes {
        octupuses[y][x].inc();
    }

    has_flash
}

fn print(octopuses: &Vec<Vec<Octopus>>) {
    octopuses.iter().for_each(|line| {
        line.iter().for_each(|octopus| print!("{:0>2}{}", octopus.life.to_string(), if octopus.must_flash { "*"} else { " "}));
        println!();
    })
}

fn expend(octupuses: &mut Vec<Vec<Octopus>>) -> bool {

    octupuses.iter_mut().for_each(|line|
        line.iter_mut().for_each(|octoppus| octoppus.inc())
    );


    while flashes(octupuses) {
    }
    let nb_flash:u8 = octupuses.iter_mut().flat_map(|line|
        line.iter_mut().map(|octoppus| octoppus.count_flash())
    ).sum();

    nb_flash == 100


}

pub fn execute(input: &str) {
    let mut octopuses: Vec<Vec<Octopus>> = input.split("\n").map(|v| v.to_string())
        .map(|line|

             line.chars().map(|char| char.to_digit(10).unwrap())
                 .map(|digit| Octopus {
                     life: digit as u8,
                     count_flash: 0,
                     must_flash: false
                 })
                 .collect()
        )
        .collect();

    let mut counter = 0;

    while !expend(&mut octopuses) || counter < 100 {

        counter = counter + 1;
        if counter == 100 {

            let step1_result: u128 = octopuses.iter().flat_map(|line|
                line.iter().map(|octopus| octopus.count_flash)
            ).sum();

            println!("Step 1 result : {}", step1_result)
        }
    }

    println!("Step 2 result {}", counter + 1);
}
