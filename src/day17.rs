use std::cmp::max;

#[derive(Debug, Clone)]
struct Path {
    highest_y: i32,
    current_pos: (i32, i32),
    current_velocity: (i32, i32),
}

impl Path {
    fn next(self, target_x: (i32, i32), target_y: (i32, i32)) -> Path {
        if self.is_in_target(target_x, target_y) {
            return self
        }
        let (x, y) = self.current_pos;
        let (vel_x, vel_y) = self.current_velocity;
        let (next_x, next_y) = (x + vel_x, y + vel_y);
        let (next_vel_x, next_vel_y) = (
            if vel_x < 0 {
                vel_x + 1
            } else if vel_x > 0 {
                vel_x - 1
            } else {
                0
            },
            vel_y - 1
        );
        Path {
            highest_y: max(self.highest_y, next_y),
            current_pos: (next_x, next_y),
            current_velocity: (next_vel_x, next_vel_y),
        }
    }

    fn is_fail(&self, (min_x, max_x): (i32, i32), (min_y, _): (i32, i32)) -> bool {
        let (vel_x, _) = self.current_velocity;
        let (x, y) = self.current_pos;
        y < min_y ||
            vel_x < 0 && x < min_x
            || vel_x > 0 && x > max_x
            || vel_x == 0 && (x < min_x || x > max_x)
    }

    fn is_in_target(&self, (min_x, max_x): (i32, i32), (min_y, max_y): (i32, i32)) -> bool {
        let (x, y) = self.current_pos;
        x >= min_x && x <= max_x && y >= min_y && y <= max_y
    }
}


pub fn execute(input: &str) {
    let line = input.split("\n").map(|v| v.to_string())
        .filter(|line| !line.is_empty())
        .next().unwrap();

    let ends_line = line.split(":").nth(1).unwrap();

    let pos = ends_line.split(",")
        .map(|data| data.split("=").nth(1).unwrap())
        .map(|data| data.split("..")
        .map(|pos| pos.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    let target_x = (pos[0][0], pos[0][1]);
    let target_y = (pos[1][0], pos[1][1]);

    let mut possibilities: Vec<_> = (0..=target_x.1).flat_map(|x|
        (0..=(-target_y.0)).map(move |y|
            Path {
                highest_y: 0,
                current_pos: (0, 0),
                current_velocity: (x, y),
            }
        )).collect();

    while possibilities.iter().any(|path| !path.is_in_target(target_x, target_y)) {
        possibilities = possibilities.into_iter().map(|path| path.next(target_x, target_y))
            .filter(|path| !path.is_fail(target_x, target_y))
            .collect();

    }

    let max_y = possibilities.iter().map(|path| path.highest_y).max().unwrap();

    println!("Step 1 : {}", max_y);
}