use std::cmp::max;

#[derive(Debug, Clone)]
enum Member {
    Number(u8),
    Pair(Box<Pair>),
}

impl Member {
    fn split(self, depth: usize) -> (Member, bool) {
        match self {
            Member::Number(d) if d >= 10 => {
                let left_number = d / 2;
                let right_number = if left_number * 2 != d {
                    left_number + 1
                } else {
                    left_number
                };
                (Member::Pair(Box::new(Pair {
                    depth: depth + 1,
                    left: Member::Number(left_number),
                    right: Member::Number(right_number),
                })), true)
            }
            Member::Number(d) => (Member::Number(d), false),
            Member::Pair(pair) => {
                let (new_pair, splited) = pair.split();
                (Member::Pair(Box::new(new_pair)), splited)
            }
        }
    }

    fn more_depth(&self) -> Member {
        match self {
            Member::Number(d) => Member::Number(*d),
            Member::Pair(pair) => Member::Pair(Box::new(pair.more_depth()))
        }
    }

    fn magnitude(&self) -> u128 {
        match self {
            Member::Number(d) => *d as u128,
            Member::Pair(pair) => pair.magnitude()
        }
    }
}

#[derive(Debug, Clone)]
struct Pair {
    depth: usize,
    left: Member,
    right: Member,
}

impl Pair {
    fn add_first_left(&self, d: u8) -> Member {
        let new_left = match &self.left {
            Member::Number(old_d) => {
                Member::Number(old_d + d)
            }
            Member::Pair(pair) => {
                pair.add_first_left(d)
            }
        };
        Member::Pair(Box::new(Pair {
            depth: self.depth,
            left: new_left,
            right: self.right.clone(),
        }))
    }

    fn add_first_right(&self, d: u8) -> Member {
        let new_right = match &self.right {
            Member::Number(old_d) => {
                Member::Number(old_d + d)
            }
            Member::Pair(pair) => {
                pair.add_first_right(d)
            }
        };
        Member::Pair(Box::new(Pair {
            depth: self.depth,
            left: self.left.clone(),
            right: new_right,
        }))
    }

    fn split(self) -> (Pair, bool) {
        let (left_pair, split_left) = self.left.split(self.depth);
        let (right_pair, split_right) = if split_left {
            (self.right.clone(), false)
        } else {
            self.right.split(self.depth)
        };
        (Pair {
            depth: self.depth,
            left: left_pair,
            right: right_pair,
        }, split_left || split_right
        )
    }

    fn explode(&mut self) -> (Option<u8>, Option<u8>, bool) {
        if self.depth > 4 {
            return (match self.left {
                Member::Number(d) => Some(d),
                Member::Pair(_) => panic!()
            }, match self.right {
                Member::Number(d) => Some(d),
                Member::Pair(_) => panic!()
            }, true);
        };

        if let Member::Pair(mut pair) = self.left.clone() {
            let mut first_explode = false;
            let result = match pair.explode() {
                (left, Some(r), _) => {
                    if left.is_some() {
                        first_explode = true;
                    }
                    let new_right = match &self.right {
                        Member::Number(d) => {
                            Member::Number(d + r)
                        }
                        Member::Pair(right_pair) => {
                            right_pair.add_first_left(r)
                        }
                    };
                    self.right = new_right;
                    (left, None, true)
                }
                whatever => whatever
            };

            if first_explode {
                self.left = Member::Number(0)
            } else {
                self.left = Member::Pair(pair);
            }

            if result.2 {
                return result;
            }
        }

        if let Member::Pair(mut pair) = self.right.clone() {
            let mut first_explode = false;
            let result = match pair.explode() {
                (Some(l), right, _) => {
                    if right.is_some() {
                        first_explode = true;
                    }
                    let new_left = match &self.left {
                        Member::Number(d) => {
                            Member::Number(d + l)
                        }
                        Member::Pair(left_pair) => {
                            left_pair.add_first_right(l)
                        }
                    };
                    self.left = new_left;
                    (None, right, true)
                }
                whatever => whatever
            };

            if first_explode {
                self.right = Member::Number(0)
            } else {
                self.right = Member::Pair(pair);
            }


            if result.2 {
                return result;
            }
        };

        (None, None, false)
    }

    fn have_pairs_to_explode(&self) -> bool {
        let left_can_explode = match &self.left {
            Member::Number(_) => false,
            Member::Pair(pair) => pair.have_pairs_to_explode()
        };
        let right_can_explode = match &self.right {
            Member::Number(_) => false,
            Member::Pair(pair) => pair.have_pairs_to_explode()
        };
        left_can_explode || right_can_explode || self.depth > 4
    }

    fn print(&self) -> String {
        let left = match &self.left {
            Member::Number(d) => format!("{}", d),
            Member::Pair(pair) => pair.print()
        };
        let right = match &self.right {
            Member::Number(d) => format!("{}", d),
            Member::Pair(pair) => pair.print()
        };
        format!("[{}{},{}]", if self.depth > 4 { "*" } else { "" }, left, right)
    }

    fn reduce(self) -> Pair {
        let mut pair = self.clone();
        let mut has_splited = false;
        while pair.have_pairs_to_explode() || has_splited {
            has_splited = false;
            while pair.have_pairs_to_explode() {
                pair.explode();
            }
            let (new_pair, new_has_splited) = pair.split();
            has_splited = new_has_splited;
            pair = new_pair;
        }
        pair
    }

    fn magnitude(&self) -> u128 {
        self.left.magnitude() * 3 + self.right.magnitude() * 2
    }

    fn more_depth(&self) -> Pair {
        Pair {
            depth: self.depth + 1,
            left: self.left.more_depth(),
            right: self.right.more_depth(),
        }
    }

    fn add(&self, other: &Pair) -> Pair {
        Pair {
            depth: 1,
            left: Member::Pair(Box::new(self.more_depth())),
            right: Member::Pair(Box::new(other.more_depth())),
        }
    }
}

#[derive(Debug, Clone)]
struct Parsing {
    pub current_depth: usize,
    pub left: Option<Member>,
}


pub fn execute(input: &str) {
    let input: Vec<_> = input.split("\n").map(|v| v.to_string())
        .filter(|line| !line.is_empty())
        .collect();

    let lines: Vec<_> = input.into_iter().map(|line| {
        let mut current_parsing: Vec<Parsing> = vec![];
        for car in line.chars() {
            if car == '[' {
                let last_depth = current_parsing.last().map(|pair| pair.current_depth).unwrap_or(0);
                current_parsing.push(Parsing {
                    current_depth: last_depth + 1,
                    left: None,
                });
            } else if car == ']' {
                // do nothing
            } else if car == ',' {
                // do nothing
            } else {
                let last_parsing = current_parsing.last().unwrap().clone();
                let value = car.to_digit(10).unwrap() as u8;
                current_parsing.remove(current_parsing.len() - 1);
                match last_parsing.left {
                    None => current_parsing.push(Parsing {
                        current_depth: last_parsing.current_depth,
                        left: Some(Member::Number(value)),
                    }),
                    Some(_) => {
                        let mut current_pair = Pair {
                            depth: last_parsing.current_depth,
                            left: last_parsing.left.unwrap(),
                            right: Member::Number(value),
                        };
                        if current_parsing.is_empty() {
                            return current_pair;
                        } else {
                            let mut before_parsing = current_parsing.remove(current_parsing.len() - 1);
                            while before_parsing.left.is_some() && !current_parsing.is_empty() {
                                current_pair = Pair {
                                    depth: before_parsing.current_depth,
                                    left: before_parsing.left.unwrap(),
                                    right: Member::Pair(Box::new(current_pair)),
                                };
                                before_parsing = current_parsing.remove(current_parsing.len() - 1);
                            }
                            if before_parsing.left.is_some() {
                                return Pair {
                                    depth: before_parsing.current_depth,
                                    left: before_parsing.left.unwrap(),
                                    right: Member::Pair(Box::new(current_pair)),
                                };
                            } else {
                                current_parsing.push(Parsing {
                                    current_depth: before_parsing.current_depth,
                                    left: Some(Member::Pair(Box::new(current_pair))),
                                })
                            }
                        }
                    }
                }
            };
        }
        panic!();
    }).collect();

    let mut pairs = lines.clone();

    let mut current_pair = pairs.remove(0).reduce();

    for next_pair in pairs {
        let next = current_pair.add(&next_pair);
        current_pair = next.reduce();
    }

    println!("Step 1 magnitude {}", current_pair.magnitude());

    let tries: Vec<_> = (0..lines.len()).flat_map(|x|
        (0..lines.len()).filter(move|y| *y != x).flat_map(move|y|
            vec![(x, y), (y, x)]
        )
    ).collect();

    let max: u128 = tries.into_iter()
        .map(|(x, y)|
            lines[x].add(&lines[y])
        ).fold(0, |max_magn, pair|
        max(max_magn, pair.reduce().magnitude())
    );

    println!("Step 2 max : {}", max)

}