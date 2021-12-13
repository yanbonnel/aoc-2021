use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Cave {
    id: String,
}

struct Path {
    path: Vec<String>,
    is_final: bool,
    already_visit_2_smal: bool,
}

impl Path {
    fn display(&self) -> String {
        self.path.join(",")
    }

    fn next(self, links: &HashMap<String, Vec<String>>, step2: bool) -> Vec<Self> {
        if self.is_final {
            return vec![self];
        }
        let last_cave = self.path.iter().last().unwrap().clone();

        links.get(&last_cave).unwrap().iter().filter_map(|next| {
            let already_small = next.to_lowercase() == next.to_string() && self.path.contains(next);
            if already_small && (!step2 || (
                step2 && self.already_visit_2_smal
                ))  {
                None
            } else {
                let mut next_path = self.path.clone();
                next_path.push(next.clone());

                Some(Path {
                    path: next_path,
                    is_final: next.to_string() == "end".to_string(),
                    already_visit_2_smal: self.already_visit_2_smal || already_small
                })
            }
        }).collect()
    }
}


pub fn execute(input: &str) {
    let links: Vec<_> = input.split("\n").map(|v| v.to_string())
        .map(|line| (line.split("-").nth(0).unwrap().to_string(), line.split("-").nth(1).unwrap().to_string()))
        .collect();

    let caves: HashSet<_> = links.clone().into_iter().flat_map(|(c1, c2)|
        vec![
            c1, c2,
        ]
    ).map(|c| Cave { id: c }).collect();

    let links: HashMap<String, Vec<String>> = links.into_iter().fold(caves.iter().map(|c|
        (c.id.clone(), vec![])
    ).collect(), |mut acc, (c1, c2)| {
        acc.get_mut(&c1).unwrap().push(c2.clone());
        if c1 != "start" {
            acc.get_mut(&c2).unwrap().push(c1.clone());
        }
        acc
    });

    let mut paths = vec![Path {
        path: vec!["start".to_string()],
        is_final: false,
        already_visit_2_smal: false,
    }];

    while paths.iter().any(|path| !path.is_final) {
        paths = paths.into_iter().flat_map(|path| path.next(&links, false)).collect()
    }

    println!("Step 1 result : {}", paths.len());

    let mut paths = vec![Path {
        path: vec!["start".to_string()],
        is_final: false,
        already_visit_2_smal: false,
    }];

    while paths.iter().any(|path| !path.is_final) {
        paths = paths.into_iter().flat_map(|path| path.next(&links, true)).collect()
    }

    println!("Step 2 result : {}", paths.len());
}
