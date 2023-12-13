use std::collections::HashSet;

use crate::{benchmark_util::Solution, input_util};

pub struct Day10;

impl Solution for Day10 {
    fn part_one(&self, file_path: &str) -> i64 {
        let lines: Vec<String> = input_util::read_file_buffered(file_path)
            .flatten()
            .collect();
        let tile_rows: Vec<Vec<Tile>> = lines
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| Tile::new(x, y, char))
                    .collect()
            })
            .collect();
        let first_location: &Tile = tile_rows
            .iter()
            .flatten()
            .find(|tile| {
                matches!(tile, Tile::Start(_, _))
            })
            .unwrap();
        let mut visited: HashSet<Tile> = HashSet::new();
        let mut cursors = first_location.starting_pipes(&tile_rows);
        let mut count = 1;
        loop {
            visited.insert(cursors.0);
            visited.insert(cursors.1);
            if cursors.0 == cursors.1 {
                return count;
            }
            cursors.0 = cursors
                .0
                .next_pipe(&tile_rows, &visited)
                .expect("First pipe to be continuous");
            cursors.1 = cursors
                .1
                .next_pipe(&tile_rows, &visited)
                .expect("Second pipe to be continuous");
            count += 1;
        }
    }

    fn part_two(&self, file_path: &str) -> i64 {
        todo!()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Tile {
    Start(usize, usize),
    Pipe(usize, usize, char),
    Ground,
}

impl Tile {
    fn new(x: usize, y: usize, value: char) -> Self {
        match value {
            'S' => Self::Start(x, y),
            '.' => Self::Ground,
            pipe => Self::Pipe(x, y, pipe),
        }
    }

    fn starting_pipes(&self, list: &[Vec<Self>]) -> (Self, Self) {
        if let Tile::Start(x, y) = self {
            (list[*y + 1][*x], list[*y - 1][*x])
        } else {
            unreachable!()
        }
    }

    fn next_pipe(&self, list: &[Vec<Self>], map: &HashSet<Tile>) -> Option<Self> {
        let find_connections = |x: usize, y: usize| list.get(y).and_then(|row| row.get(x));
        match self {
            Tile::Start(_, _) => todo!(),
            Tile::Pipe(x, y, char) => {
                let try_find: Vec<(i32, i32)> = match char {
                    '-' => vec![(1, 0), (-1, 0)],
                    'L' => vec![(1, 0), (0, -1)],
                    '|' => vec![(0, 1), (0, -1)],
                    'F' => vec![(1, 0), (0, 1)],
                    '7' => vec![(-1, 0), (0, 1)],
                    'J' => vec![(-1, 0), (0, -1)],
                    bruh => unreachable!("{}", bruh),
                };
                try_find
                    .into_iter()
                    .flat_map(|pair| find_connections(*x + pair.0 as usize, *y + pair.1 as usize))
                    .find(|tile| {
                        matches!(map.get(tile), None)
                    })
                    .copied()
            }
            Tile::Ground => todo!(),
        }
    }
}
