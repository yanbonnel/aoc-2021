
enum Action {
    Forward(i32),
    Up(i32),
    Down(i32)
}

fn parse(line: &str) -> Action {
    let splitted: Vec<_> = line.split(" ").collect();
    let key = splitted[0];
    let nb = splitted[1].parse::<i32>().unwrap();
    match key {
        "forward" => Action::Forward(nb),
        "up" => Action::Up(nb),
        "down" => Action::Down(nb),
        _ => panic!("Action unknown : {}", key)
    }
}

pub fn execute(input: &str) {
    let values: Vec<_> = input.split("\n").map(parse).collect();
    let (x, depth) = values.iter().fold((0, 0), |(x, depth), action| {
        match action {
            Action::Forward(nb) => (x+nb, depth),
            Action::Up(nb) => (x, depth - nb),
            Action::Down(nb) => (x, depth + nb),
        }
    });
    println!("Step 1 : {}", x * depth);

    let (x, depth, _) = values.iter().fold((0, 0, 0), |(x, depth, aim), action| {
        let result = match action {
            Action::Forward(nb) => (x + nb, depth + aim*nb, aim),
            Action::Up(nb) => (x, depth, aim - nb),
            Action::Down(nb) => (x, depth, aim + nb),
        };
        result
    });
    println!("Step 2 : {}", x * depth);




}