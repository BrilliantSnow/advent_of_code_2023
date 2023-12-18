use crate::{benchmark_util::Solution, input_util};

pub struct Day11;

type GalaxyCoords = (usize, usize);

fn galaxy_distances(file_path: &str, universe_width: usize, expansion_rate: usize) -> i64 {
    let lines: Vec<Vec<char>> = input_util::read_file_buffered(file_path)
        .flatten()
        .map(|line| line.chars().collect())
        .collect();
    let row_has_no_galaxy: Vec<bool> = lines
        .iter()
        .map(|row| row.iter().all(|letter| '#'.ne(letter)))
        .collect();
    let column_has_no_galaxy: Vec<bool> = (0..universe_width)
        .map(|x| lines.iter().all(|row| '#'.ne(row.get(x).unwrap())))
        .collect();
    let galaxies: Vec<GalaxyCoords> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let y_expansion =
                row_has_no_galaxy[0..y].iter().filter(|row| **row).count() * (expansion_rate - 1);
            row.iter()
                .enumerate()
                .filter(|(_, letter)| '#'.eq(letter))
                .map(|(x, _)| {
                    let x_expansion = column_has_no_galaxy[0..x]
                        .iter()
                        .filter(|col: &&bool| **col)
                        .count()
                        * (expansion_rate - 1);
                    (x + x_expansion, y + y_expansion)
                })
                .collect::<Vec<GalaxyCoords>>()
        })
        .collect();
    galaxies
        .iter()
        .flat_map(|galaxy| {
            galaxies
                .iter()
                .map(|galaxy2| (galaxy, galaxy2))
                .collect::<Vec<(&GalaxyCoords, &GalaxyCoords)>>()
        })
        .map(|(left, right)| {
            (left.0.max(right.0) - left.0.min(right.0)) as i64
                + (left.1.max(right.1) - left.1.min(right.1)) as i64
        })
        .sum::<i64>()
        / 2
}

impl Solution for Day11 {
    fn part_one(&self, file_path: &str) -> i64 {
        galaxy_distances(file_path, 140, 2)
    }

    fn part_two(&self, file_path: &str) -> i64 {
        galaxy_distances(file_path, 140, 1_000_000)
    }
}

#[test]
fn test_file_expansion_10() {
    let distance = galaxy_distances("src/res/day_11_test.txt", 10, 10);
    assert_eq!(distance, 1030);
}

#[test]
fn test_file_expansion_100() {
    let distance = galaxy_distances("src/res/day_11_test.txt", 10, 100);
    assert_eq!(distance, 8410);
}
