use std::fmt;
use itertools::Itertools;

advent_of_code::solution!(11);

struct Map {
    nodes: Vec<char>,
    width: usize,
    height: usize
}

impl Map {
    fn get_char(&self, pos: &Vector2i) -> Option<&char> {
        match self.is_inside(pos) {
            false => None,
            true => {
                let index = pos.x.unsigned_abs() + pos.y.unsigned_abs() * self.width;
                Some(&self.nodes[index])
            }
        }
    }

    fn is_inside(&self, pos: &Vector2i) -> bool {
        pos.x >= 0 && pos.x < self.width.try_into().unwrap() && pos.y >= 0 && pos.y < self.height.try_into().unwrap()
    }
}

#[derive(Clone)]
struct Vector2i {
    x: isize,
    y: isize
}

impl Vector2i {
    fn manhattan_distance(&self, other_vec: &Vector2i) -> usize {
        ((self.x - other_vec.x).abs() + (self.y - other_vec.y).abs()).unsigned_abs()
    }
}

impl fmt::Debug for Vector2i {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    println!("{}", input);
    let mut nodes : Vec<char> = Vec::new();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    input.lines().map(|line| line.chars().collect::<Vec<_>>()).for_each(|arr| {
        arr.iter().for_each(|c| nodes.push(c.clone()));
    });
    let map = Map {
        nodes, width, height
    };
    println!("{:?}", map.nodes);
    let mut galaxy_positions : Vec<Vector2i> = Vec::new();
    // Get a list of all columns and all rows that will expand
    let mut expanding_cols: Vec<usize> = Vec::new();
    for x in 0..map.width {
        let mut did_find_galaxy = false;
        for y in 0..map.height {
            let pos = Vector2i{ x: x.try_into().unwrap(), y: y.try_into().unwrap()};
            match map.get_char(&pos) {
                None => {},
                Some(c) => {
                    match c {
                        '#' => { 
                            galaxy_positions.push(pos);
                            did_find_galaxy = true;
                        },
                        _ => {}
                        
                    }
                }
            };
        }
        if !did_find_galaxy {
            expanding_cols.push(x.clone());
        }
    }
    let mut expanding_rows : Vec<usize> = Vec::new();
    for y in 0..map.height {
        let mut did_find_galaxy = false;
        for x in 0..map.width {
            let pos = Vector2i{ x: x.try_into().unwrap(), y: y.try_into().unwrap()};
            match map.get_char(&pos) {
                None => {},
                Some(c) => {
                    match c {
                        '#' => { 
                            did_find_galaxy = true;
                        },
                        _ => {}
                        
                    }
                }
            };

        }
        if !did_find_galaxy {
            expanding_rows.push(y.clone());
        }
    }
    println!("{:?}", galaxy_positions);
    println!("{:?}", expanding_cols);
    println!("{:?}", expanding_rows);
    // Now modify the galaxy positions by expanding the universe
    let mut x_adds = Vec::with_capacity(map.width);
    let mut adds = 0;
    let mut expanding_cols_iter = expanding_cols.iter();
    let mut next_col = expanding_cols_iter.next();
    for i in 0..map.width {
        if next_col.is_some() {
            if *next_col.unwrap() == i {
                adds += 1;
                next_col = expanding_cols_iter.next();
            }
        }
        x_adds.push(adds);
    }

    let mut y_adds = Vec::with_capacity(map.height);
    adds = 0;
    let mut expanding_rows_iter = expanding_rows.iter();
    let mut next_row = expanding_rows_iter.next();
    for i in 0..map.height {
        if next_row.is_some() {
            if *next_row.unwrap() == i {
                adds += 1;
                next_row = expanding_rows_iter.next();
            }
        }
        y_adds.push(adds);
    }
    let mut new_galaxy_positions : Vec<Vector2i> = Vec::with_capacity(galaxy_positions.capacity());
    for old_galaxy in galaxy_positions.iter() {
        let mut new_galaxy_pos = old_galaxy.clone();
        new_galaxy_pos.x += x_adds[old_galaxy.x.unsigned_abs()];
        new_galaxy_pos.y += y_adds[old_galaxy.y.unsigned_abs()];
        new_galaxy_positions.push(new_galaxy_pos);
    }
    println!("{:?}", x_adds);
    println!("{:?}", y_adds);
    println!("{:?}", new_galaxy_positions);

    // Start calculating manhattan distance between all pairs
    let mut distances_sum = 0;
    new_galaxy_positions.iter().tuple_combinations().for_each(|(a, b)| distances_sum += a.manhattan_distance(b));
    Some(distances_sum.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    println!("{}", input);
    let mut nodes : Vec<char> = Vec::new();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    input.lines().map(|line| line.chars().collect::<Vec<_>>()).for_each(|arr| {
        arr.iter().for_each(|c| nodes.push(c.clone()));
    });
    let map = Map {
        nodes, width, height
    };
    println!("{:?}", map.nodes);
    let mut galaxy_positions : Vec<Vector2i> = Vec::new();
    // Get a list of all columns and all rows that will expand
    let mut expanding_cols: Vec<usize> = Vec::new();
    for x in 0..map.width {
        let mut did_find_galaxy = false;
        for y in 0..map.height {
            let pos = Vector2i{ x: x.try_into().unwrap(), y: y.try_into().unwrap()};
            match map.get_char(&pos) {
                None => {},
                Some(c) => {
                    match c {
                        '#' => { 
                            galaxy_positions.push(pos);
                            did_find_galaxy = true;
                        },
                        _ => {}
                        
                    }
                }
            };
        }
        if !did_find_galaxy {
            expanding_cols.push(x.clone());
        }
    }
    let mut expanding_rows : Vec<usize> = Vec::new();
    for y in 0..map.height {
        let mut did_find_galaxy = false;
        for x in 0..map.width {
            let pos = Vector2i{ x: x.try_into().unwrap(), y: y.try_into().unwrap()};
            match map.get_char(&pos) {
                None => {},
                Some(c) => {
                    match c {
                        '#' => { 
                            did_find_galaxy = true;
                        },
                        _ => {}
                        
                    }
                }
            };

        }
        if !did_find_galaxy {
            expanding_rows.push(y.clone());
        }
    }
    let expansion_factor = 999999;
    println!("{:?}", galaxy_positions);
    println!("{:?}", expanding_cols);
    println!("{:?}", expanding_rows);
    // Now modify the galaxy positions by expanding the universe
    let mut x_adds = Vec::with_capacity(map.width);
    let mut adds = 0;
    let mut expanding_cols_iter = expanding_cols.iter();
    let mut next_col = expanding_cols_iter.next();
    for i in 0..map.width {
        if next_col.is_some() {
            if *next_col.unwrap() == i {
                adds += expansion_factor;
                next_col = expanding_cols_iter.next();
            }
        }
        x_adds.push(adds);
    }

    let mut y_adds = Vec::with_capacity(map.height);
    adds = 0;
    let mut expanding_rows_iter = expanding_rows.iter();
    let mut next_row = expanding_rows_iter.next();
    for i in 0..map.height {
        if next_row.is_some() {
            if *next_row.unwrap() == i {
                adds += expansion_factor;
                next_row = expanding_rows_iter.next();
            }
        }
        y_adds.push(adds);
    }
    let mut new_galaxy_positions : Vec<Vector2i> = Vec::with_capacity(galaxy_positions.capacity());
    for old_galaxy in galaxy_positions.iter() {
        let mut new_galaxy_pos = old_galaxy.clone();
        new_galaxy_pos.x += x_adds[old_galaxy.x.unsigned_abs()];
        new_galaxy_pos.y += y_adds[old_galaxy.y.unsigned_abs()];
        new_galaxy_positions.push(new_galaxy_pos);
    }
    println!("{:?}", x_adds);
    println!("{:?}", y_adds);
    println!("{:?}", new_galaxy_positions);

    // Start calculating manhattan distance between all pairs
    let mut distances_sum = 0;
    new_galaxy_positions.iter().tuple_combinations().for_each(|(a, b)| {
        distances_sum += a.manhattan_distance(b);
    });
    Some(distances_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
