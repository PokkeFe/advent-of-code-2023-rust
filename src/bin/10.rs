use core::slice;

advent_of_code::solution!(10);

#[derive(Debug,Clone)]
enum NodeState {
    Pipe(Directions),
    Ground,
    Animal
}

#[derive(Debug,Clone)]
struct Directions {
    north: bool,
    east: bool,
    south: bool,
    west: bool
}

impl Directions {
    fn from(north: bool, east: bool, south: bool, west: bool) -> Directions {
        Directions {
            north,
            east,
            south,
            west
        }
    }

    fn has(&self, dir: Direction) -> bool {
        match dir {
            Direction::NORTH => self.north,
            Direction::EAST => self.east,
            Direction::SOUTH => self.south,
            Direction::WEST => self.west
        }
    }

    fn get_dir_vector(&self) -> Vec<Direction> {
        let mut vec = Vec::new();

        if self.north { vec.push(Direction::NORTH) }
        if self.east { vec.push(Direction::EAST) }
        if self.south { vec.push(Direction::SOUTH) }
        if self.west { vec.push(Direction::WEST) }

        vec
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST
}

impl Direction {
    fn get_opposite(&self) -> Direction {
        match self {
            Self::NORTH => Self::SOUTH,
            Self::EAST => Self::WEST,
            Self::WEST => Self::EAST,
            Self::SOUTH => Self:: NORTH
        }
    }

    fn get_next_coords(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::NORTH => (x, y.wrapping_sub(1)),
            Direction::EAST => (x + 1, y),
            Direction::SOUTH => (x, y + 1),
            Direction::WEST => (x.wrapping_sub(1), y)
        }
    }
}

struct Map {
    size: (usize, usize),
    start_position: (usize, usize),
    nodes: Vec<NodeState>
}

impl Map {
    fn width(&self) -> usize { self.size.0 }
    fn height(&self) -> usize { self.size.1 }

    fn get_node(&self, x: usize, y:usize) -> Option<&NodeState> {
        if x >= self.width() || y >= self.height() {
            return None;
        }

        self.nodes.get(x + (y * self.width()))
    }

    fn is_pipe_pointing(&self, x: usize, y: usize, dir: Direction) -> bool {
        let node = self.get_node(x, y);
        if node.is_none() {
            false
        } else {
            match node.unwrap() {
                NodeState::Pipe(pipe_dir) => {
                    pipe_dir.has(dir)
                },
                _ => false
            }
        }

    }

    fn get_node_symbol(&self, position: (usize, usize)) -> &str {
        match self.get_node(position.0, position.1) {
            None => "?",
            Some(node_state) => {
                match node_state {
                    NodeState::Ground => ".",
                    NodeState::Animal => "S",
                    NodeState::Pipe(pipe_dir) => {
                        match (pipe_dir.north, pipe_dir.east, pipe_dir.south, pipe_dir.west) {
                            (true, true, false, false) => "L",
                            (true, false, true, false) => "|",
                            (true, false, false, true) => "J",
                            (false, true, true, false) => "F",
                            (false, true, false, true) => "-",
                            (false, false, true, true) => "7",
                            _ => "?"
                        }
                    }
                }
            }
        }
    }

    fn build(input: &str) -> Map {
        let mut nodes = Vec::new();
        let width: usize = input.lines().next().unwrap().len();
        let height: usize = input.lines().count();
        let mut start_position: (usize, usize) = (0, 0);

        input.lines().for_each(|line| {
            line.chars().for_each(|char| {
                let node = match char {
                    '-' => NodeState::Pipe(Directions::from(false, true, false, true)),
                    '|' => NodeState::Pipe(Directions::from(true, false, true, false)),
                    'F' => NodeState::Pipe(Directions::from(false, true, true, false)),
                    '7' => NodeState::Pipe(Directions::from(false, false, true, true)),
                    'L' => NodeState::Pipe(Directions::from(true, true, false, false)),
                    'J' => NodeState::Pipe(Directions::from(true, false, false, true)),
                    'S' => {
                        start_position = (nodes.len() % width, nodes.len() / width);
                        NodeState::Animal
                    },
                    _ => NodeState::Ground
                };
                nodes.push(node);
            });
        });
        Map { 
            size: (width, height),
            start_position,
            nodes
        }
    }

    fn empty(width: usize, height: usize) -> Map {
        let mut nodes = Vec::new();
        for i in 0..width*height {
            nodes.push(NodeState::Ground);
        }
        Map {
            size: (width, height),
            start_position: (0, 0),
            nodes
        }
    }

    fn print(&self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                print!("{}", self.get_node_symbol((x, y)))
            }
            println!("");
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    // Construct the map
    println!("Building Map");
    let map = Map::build(input);
    println!("Map Built");
    println!("Traversing Paths");

    let mut agent_position = map.start_position.clone();
    let mut agent_entry_direction: Direction;
    
    let mut final_agent_steps = 0;

    let valid_dirs: Vec<Direction> = vec![Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST].into_iter().filter(|d| {
        let next_coords = d.get_next_coords(map.start_position.0, map.start_position.1);
        match map.get_node(next_coords.0, next_coords.1) {
            None => false,
            Some(node_state) => {
                match node_state {
                    NodeState::Pipe(pd) => {
                        pd.has(d.get_opposite())
                    },
                    _ => false
                }
            }
        }
    }).collect();
    for (i, dir) in valid_dirs.iter().enumerate() {
        // For each possible direction, run the agent until it fails. If the agent hits home again, we've found the cycle
        agent_position = map.start_position.clone();
        agent_position = dir.get_next_coords(agent_position.0, agent_position.1);
        agent_entry_direction = dir.get_opposite();
        final_agent_steps = 1;
        let mut made_it_home: bool = false;
        loop {
            // Check to see if the next position is valid. if it isn't, this path is a fail. If it's the animal again, we're done and have our cycle
            match map.get_node(agent_position.0, agent_position.1) {
                None => {
                    // We failed. Break the loop.
                    break;
                },
                Some(node_state) => {
                    match node_state {
                        NodeState::Pipe(pipe_dirs) => {
                            let out_dir = pipe_dirs.get_dir_vector().into_iter().filter(|check_dir| *check_dir != agent_entry_direction).last().unwrap();
                            let next_coords = out_dir.get_next_coords(agent_position.0, agent_position.1);
                            let next_node_state = map.get_node(next_coords.0, next_coords.1);
                            match next_node_state {
                                None => {
                                    //Out of map, failed.
                                    break;
                                },
                                Some(ns) => {
                                    match ns {
                                        NodeState::Animal => {
                                            made_it_home = true;
                                            break;
                                        },
                                        NodeState::Ground => {
                                            break;
                                        },
                                        NodeState::Pipe(next_pipe_dir) => {
                                            if next_pipe_dir.has(out_dir.get_opposite()) {
                                                agent_position = next_coords;
                                                agent_entry_direction = out_dir.get_opposite();
                                                final_agent_steps += 1;
                                                continue;
                                            } else {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        _ => {
                            panic!("Something went wrong. Agent offroading.");
                        }
                    }
                }
            };
        }
        if made_it_home {
            break;
        }
    }
    
    Some((final_agent_steps + 1) / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Construct the map
    println!("Building Map");
    let map = Map::build(input);
    println!("Map Built");

    println!("Traversing Paths");
    let mut agent_position = map.start_position.clone();
    let mut agent_entry_direction: Direction;

    let mut main_loop_points: Vec<(usize, usize)> = Vec::new();

    let valid_dirs: Vec<Direction> = vec![Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST].into_iter().filter(|d| {
        let next_coords = d.get_next_coords(map.start_position.0, map.start_position.1);
        match map.get_node(next_coords.0, next_coords.1) {
            None => false,
            Some(node_state) => {
                match node_state {
                    NodeState::Pipe(pd) => {
                        pd.has(d.get_opposite())
                    },
                    _ => false
                }
            }
        }
    }).collect();
    for (i, dir) in valid_dirs.iter().enumerate() {
        // For each possible direction, run the agent until it fails. If the agent hits home again, we've found the cycle
        agent_position = map.start_position.clone();
        agent_position = dir.get_next_coords(agent_position.0, agent_position.1);
        agent_entry_direction = dir.get_opposite();
        let mut visited_points: Vec<(usize, usize)> = Vec::new();
        visited_points.push(map.start_position.clone());
        let mut made_it_home: bool = false;
        loop {
            visited_points.push(agent_position);
            // Check to see if the next position is valid. if it isn't, this path is a fail. If it's the animal again, we're done and have our cycle
            match map.get_node(agent_position.0, agent_position.1) {
                None => {
                    // We failed. Break the loop.
                    break;
                },
                Some(node_state) => {
                    match node_state {
                        NodeState::Pipe(pipe_dirs) => {
                            let out_dir = pipe_dirs.get_dir_vector().into_iter().filter(|check_dir| *check_dir != agent_entry_direction).last().unwrap();
                            let next_coords = out_dir.get_next_coords(agent_position.0, agent_position.1);
                            let next_node_state = map.get_node(next_coords.0, next_coords.1);
                            match next_node_state {
                                None => {
                                    //Out of map, failed.
                                    break;
                                },
                                Some(ns) => {
                                    match ns {
                                        NodeState::Animal => {
                                            made_it_home = true;
                                            break;
                                        },
                                        NodeState::Ground => {
                                            break;
                                        },
                                        NodeState::Pipe(next_pipe_dir) => {
                                            if next_pipe_dir.has(out_dir.get_opposite()) {
                                                agent_position = next_coords;
                                                agent_entry_direction = out_dir.get_opposite();
                                                continue;
                                            } else {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        },
                        _ => {
                            panic!("Something went wrong. Agent offroading.");
                        }
                    }
                }
            };
        }
        if made_it_home {
            main_loop_points = visited_points;
            break;
        }
    };

    println!("Building Clean Map");
    let mut clean_map = Map::empty(map.width(), map.height());
    for point in main_loop_points {
        let index = point.0 + point.1 * clean_map.width();
        clean_map.nodes[index] = map.nodes[index].clone();
    }
    clean_map.start_position = map.start_position.clone();

    // Fix the start potision to be a pipe
    {
        let (x, y) = clean_map.start_position;
        let north = clean_map.is_pipe_pointing(x, y.wrapping_sub(1), Direction::SOUTH);
        let east = clean_map.is_pipe_pointing(x+1, y, Direction::WEST);
        let south = clean_map.is_pipe_pointing(x, y+1, Direction::NORTH);
        let west = clean_map.is_pipe_pointing(x.wrapping_sub(1), y, Direction::EAST);
        let index = x + (clean_map.width() * y);
        clean_map.nodes[index] = NodeState::Pipe(Directions {
            north, south, east, west
        })
    }

    clean_map.print();

    let mut inside_points: usize = 0;
    for index in 0..clean_map.width()*clean_map.height() {
        // iterate over each point and check if it's inside
        let mut intersections = 0;
        let mut x = index % clean_map.width();
        let mut y = index / clean_map.width();
        if clean_map.get_node_symbol((x, y)) != "." {continue}
        while x > 0 && y > 0 {
            x -= 1;
            y -= 1;
            let symbol = clean_map.get_node_symbol((x, y));
            match symbol {
                "-" | "|" | "F" | "J" => {intersections += 1}
                _ => {}
            };
        }
        if intersections % 2 == 1 {
            inside_points += 1;
        }
    }

    Some(inside_points.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_map() {
        let input = "S-7
|.|
L-J";
        let map = Map::build(input);
        let values = vec![2, 1, 1, 1, 0, 1, 1, 1, 1];
        for i in 0..9 {
            assert!(match map.nodes[i] {
                NodeState::Ground => values[i] == 0,
                NodeState::Pipe(_) => values[i] == 1,
                NodeState::Animal => values[i] == 2
            }, "Map Value Mismatch");
        }
        assert_eq!(map.size, (3, 3), "Map Size Mismatch");
        assert_eq!(map.start_position, (0, 0), "Map Start Position incorrect");
    }

    #[test]
    fn test_get_node() {
        let input = "S-7
|.|
L-J";
        let map = Map::build(input);
        let values = vec![2, 1, 1, 1, 0, 1, 1, 1, 1];
        for x in 0..3 {
            for y in 0..3 {
                assert!(match map.get_node(x, y) {
                    Some(NodeState::Ground) => values[x + y * map.width()] == 0,
                    Some(NodeState::Pipe(_)) => values[x + y * map.width()] == 1,
                    Some(NodeState::Animal) => values[x + y * map.width()] == 2,
                    None => false
                }, "Map Value Mismatch");
            } 
        }

    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
