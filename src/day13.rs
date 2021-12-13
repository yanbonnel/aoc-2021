use std::collections::HashSet;


pub fn fold(fold_instruction: String, points: HashSet<(u32, u32)>) -> HashSet<(u32, u32)> {
    let fold_number: u32 = fold_instruction.split("=").nth(1).unwrap().parse().unwrap();
    let is_x_fold = fold_instruction.split("=").nth(0).unwrap().ends_with("x");
    if is_x_fold {
        points.into_iter().filter_map(|(x, y)| {
            if x == fold_number {
                None
            } else if x < fold_number {
                Some((x, y))
            } else {
                Some((fold_number - (x - fold_number), y))
            }
        }).collect()
    } else {
        points.into_iter().filter_map(|(x, y)| {
            if y == fold_number {
                None
            } else if y < fold_number {
                Some((x, y))
            } else {
                Some((x, fold_number - (y - fold_number)))
            }
        }).collect()
    }
}

pub fn execute(input: &str) {
    let values: Vec<_> = input.split("\n").map(|v| v.to_string())
        .filter(|line| !line.is_empty())
        .collect();

    let mut points: HashSet<(u32, u32)> = HashSet::new();

    let mut folds: Vec<String> = vec![];

    for value in values {
        if value.starts_with("fold") {
            folds.push(value);
        } else {
            points.insert(
                (
                    value.split(",").nth(0).unwrap().parse().unwrap(),
                    value.split(",").nth(1).unwrap().parse().unwrap()
                )
            );
        }
    }

    let step1_fold = fold(folds[0].clone(), points.clone());
    println!("Step 1 result : {}", step1_fold.len());


    let step2_fold = folds.into_iter().fold(points, |acc, fold_instruction| fold(fold_instruction, acc));

    let (max_x, max_y) = (
        step2_fold.iter().map(|(x, y)| *x).max().unwrap(),
        step2_fold.iter().map(|(x, y)| *y).max().unwrap()
    );

    for y in 0..=max_y {
        for x in 0..=max_x {
            if step2_fold.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }



}
