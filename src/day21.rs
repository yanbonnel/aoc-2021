use std::cmp::max;
use std::collections::{HashMap, HashSet};


struct Die {
    current: u128,
}

impl Die {
    fn _next(&mut self) -> u16 {
        let result = self.current % 100 + 1;
        self.current = self.current + 1;
        if result == 0 {
            100
        } else {
            result as u16
        }
    }

    fn next_run(&mut self) -> u16 {
        self._next() + self._next() + self._next()
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Player {
    id: u8,
    pos: u16,
    score: u128,
}

#[derive(Debug, Clone)]
struct Board {
    players: Vec<Player>,
    count: u128,
}

impl Board {
    fn is_finish(&self) -> bool {
        self.players.iter().any(|player| player.score >= 21)
    }

    fn next(self) -> Vec<Self> {
        if self.is_finish() {
            vec![self]
        } else {
            let next_player = self.players.get(0).unwrap();
            let other_player = self.players.get(1).unwrap();
            all_dies().into_iter().map(|(die, count)| {
                Board {
                    players: vec![other_player.clone(), next_player.play(die)],
                    count: self.count * count
                }
            }).collect()
        }
    }
}

fn all_dies() -> Vec<(u16, u128)> {
    vec![
        (3, 1),
        (4, 3),
        (5, 6),
        (6, 7),
        (7, 6),
        (8, 3),
        (9, 1)
    ]
}

impl Player {
    fn win(&self) -> bool {
        self.score >= 1000
    }

    fn play(&self, die: u16) -> Player {
        let next_pos = (self.pos + die - 1) % 10 + 1;
        Player {
            id: self.id,
            pos: next_pos,
            score: self.score + next_pos as u128,
        }
    }
}


pub fn execute(input: &str) {
    let mut players = vec![
        Player {
            id: 1,
            pos: 2,
            score: 0,
        },
        Player {
            id: 2,
            pos: 8,
            score: 0,
        },
    ];

    let mut die = Die {
        current: 0
    };

    while players.iter().all(|p| !p.win()) {
        let next_player = players.remove(0);
        let roll = die.next_run();
        let next_player = next_player.play(roll);

        players.push(next_player);
    }

    let loser = players.get(0).unwrap().clone();
    let result = loser.score * die.current;

    println!("Step 1 result {}", result);


    let mut boards = vec![Board {
        players: vec![
            Player {
                id: 1,
                pos: 2,
                score: 0,
            },
            Player {
                id: 2,
                pos: 8,
                score: 0,
            },
        ],
        count: 1,
    }];

    while boards.iter().any(|board| !board.is_finish()) {
        boards = boards.into_iter().flat_map(|board| board.next())
            .fold(HashMap::new(), |mut acc: HashMap<Vec<Player>, u128>, board| {
                let nb = acc.get(&board.players).cloned().unwrap_or(0);
                acc.insert(board.players, nb + board.count);
                acc
            }).into_iter().map(|(players, count)|
            Board {
                players,
                count
            }
        ).collect();
    }

    let players_win = boards.into_iter().map(|board| (
        (board.players.iter().find(|player| player.score >= 21).unwrap().id, board.count)
    )).fold(HashMap::new(), |mut acc: HashMap<u8, u128>, (id, count)| {
        let prec_count = acc.get(&id).cloned().unwrap_or(0);
        acc.insert(id, prec_count + count);
        acc
    });
    let winers = players_win.into_iter().max_by(|(_, a), (_, b)|
        a.cmp(b)
    ).unwrap();

    println!("Winner : {:?}", winers)


}