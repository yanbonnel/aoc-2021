use std::fmt::Debug;

#[derive(Debug, Clone)]
struct Number {
    number: i32,
    marked: bool,
}

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Vec<Number>>,
}

impl Board {
    fn new(lines: Vec<String>) -> Self {
        Board {
            board: lines.into_iter().map( | line|
                line.split(' ').filter(|nb| !nb.is_empty())
                    .map(|nb| Number {number: nb.parse::<i32>().unwrap(), marked: false})
                    .collect()
            ).collect()
        }
    }

    fn add_number(&mut self, number: i32) {
        self.board.iter_mut().for_each(|line|
            line.iter_mut().for_each(|case|
                if case.number == number {
                    case.marked = true
                }
            )
        );
    }

    fn is_win(&self) -> bool {
        let has_line_win = self.board.iter().any(|line| line.iter().all(|case| case.marked));
        let has_column_win = (0..5).any(|column_number| self.board.iter().all(|line| line[column_number].marked));
        has_column_win || has_line_win
    }

    fn score(&self) -> i32 {
        self.board.iter().flat_map(|line| line.iter()).filter(|case| !case.marked).map(|case| case.number).sum()
    }
}

pub fn execute(input: &str) {

    let values: Vec<_> = input.split("\n").map(|v| v.to_string()).collect();


    let numbers: Vec<i32> = values[0].split(',').map(|v| v.parse().unwrap()).collect();

    let boards = values.into_iter().skip(1).collect::<Vec<_>>();

    let boards: Vec<_> = (0..boards.len()).step_by(6).map(|index|
        Board::new((1..=5).map(|incr| index+incr).map(|idx| boards[idx].clone()).collect())
    ).collect();

    let score_step1 = play_step1(&numbers, &boards);

    println!("Step1 score : {}", score_step1);


    let score_step2 = play_step2(&numbers, &boards);

    println!("Step2 score : {}", score_step2);




}

fn play_step1(numbers: &Vec<i32>, boards: &Vec<Board>) -> i32 {
    let mut boards = boards.clone();
    let numbers = numbers.clone();
    for number in numbers {
        boards.iter_mut().for_each(|board| board.add_number(number));
        if let Some(win_board) = boards.iter().filter(|board| board.is_win()).next() {
            return win_board.score() * number
        }
    }
    return 0
}

fn play_step2(numbers: &Vec<i32>, boards: &Vec<Board>) -> i32 {
    let mut boards = boards.clone();
    let numbers = numbers.clone();
    for number in numbers {
        boards.iter_mut().for_each(|board| board.add_number(number));
        if boards.len() == 1 && boards[0].is_win() {
            return boards[0].score() * number
        }
        boards = boards.into_iter().filter(|board| !board.is_win()).collect()
    }
    return 0
}