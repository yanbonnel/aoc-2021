use std::collections::HashSet;

pub fn expend(pts: HashSet<(usize, usize)>, nines: &HashSet<(usize, usize)>, max_x: usize, max_y: usize) -> HashSet<(usize, usize)> {
    let actual_size = pts.len();
    let next_pts: HashSet<(usize, usize)> = pts.into_iter().flat_map(|(x, y)|
        vec![
            Some((x, y)),
            if x == 0 { None } else { Some((x-1, y))},
            if y == 0 { None } else { Some((x, y - 1))},
            if x + 1 == max_x { None } else { Some((x + 1, y))},
            if y + 1 == max_y { None } else { Some((x, y + 1))},
        ].into_iter().filter_map(|d| d)
            .filter(|pt| !nines.contains(pt))
    ).collect();

    if next_pts.len() == actual_size {
        next_pts
    } else {
        expend(next_pts, nines, max_x, max_y)
    }
}

pub fn execute(input: &str) {
    let values: Vec<Vec<u32>> = input.split("\n").map(|v| v.to_string())
        .map(|v|
            v.chars().map(|char| char.to_digit(10).unwrap()).collect()
        )
        .collect();

    let max_x = values[0].len();
    let max_y = values.len();

    let low_pts: Vec<(u32, usize, usize)> = (0..max_y).flat_map(|y| {
        let current_row = values.get(y).unwrap().clone();
        let row_up = if y == 0 { None } else { values.get(y - 1)};
        let row_down = values.get(y + 1);
        (0..max_x).filter_map(move|x| {
            let my_pt = *current_row.get(x).clone().unwrap();
            let other_points: Vec<_> = vec![
                row_up.and_then(|row| row.get(x).clone()), // UP
                if x == 0 { None } else {current_row.get(x - 1)}, // LEFT
                current_row.get(x + 1), // RIGHT
                row_down.and_then(|row| row.get(x)), // DOWN
            ].into_iter().filter_map(|d| d)
                .map(|d| *d)
                .collect();

            if other_points.into_iter().all(|other| other > my_pt) {
                Some((my_pt, x, y))
            } else {
                None
            }
        })
    }).collect();

    let step_1_result: u32 = low_pts.iter().map(|(pt, _, _)| pt + 1).sum();

    println!("Step 1 result : {}", step_1_result);

    let nines: HashSet<_> = values.into_iter().enumerate().flat_map(|(y, row)|
        row.into_iter().enumerate().filter_map(move |(x, pt)| {
            if pt == 9 {
                Some((x, y))
            } else {
                None
            }
        })
    ).collect();

    let mut bassins = low_pts.into_iter().map(|(_, x, y)| {
        expend(vec![(x, y)].into_iter().collect(), &nines, max_x, max_y).len()
    }).collect::<Vec<_>>();

    bassins.sort();
    bassins.reverse();

    let bassins_sizes = bassins.into_iter().take(3).fold(1, |acc: u128, bassin_size|
        acc * bassin_size as u128
    );

    println!("Step 2 result : {}", bassins_sizes);


}
