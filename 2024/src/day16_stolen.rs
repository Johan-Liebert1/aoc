use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::Display,
    ops::Add,
    usize,
};

struct Maze {
    grid: Vec<Vec<Map>>,
    start: (usize, usize),
    end: (usize, usize),
    height: usize,
    width: usize,
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let mut grid: Vec<Vec<Map>> = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (row, line) in value.lines().enumerate() {
            let mut grid_row = Vec::new();

            for (col, char) in line.char_indices() {
                if char == 'S' {
                    start = (row, col);
                }
                if char == 'E' {
                    end = (row, col)
                }

                grid_row.push(Map::from(char));
            }

            grid.push(grid_row);
        }

        let height = grid.len();
        let width = grid[0].len();

        Self {
            grid,
            start,
            end,
            height,
            width,
        }
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let string = self
        //     .grid
        //     .iter()
        //     .map(|row| {
        //         row.iter()
        //             .map(|object| char::from(object))
        //             .collect::<String>()
        //     })
        //     .join("\n");

        writeln!(f, "welp")
    }
}

impl Maze {
    fn dijkstra(&self) -> usize {
        let mut min_cost = usize::MAX;
        let mut to_visit = vec![vec![usize::MAX; self.width]; self.height];

        let mut queue = BinaryHeap::new();

        to_visit[self.start.0][self.start.1] = 0;

        queue.push(Reverse(Tile {
            position: self.start,
            direction: Direction::East,
            cost: 0,
            history: None,
        }));

        while let Some(Reverse(Tile {
            position,
            direction,
            cost,
            history: _,
        })) = queue.pop()
        {
            let (row, col) = position;

            if position == self.end && cost < min_cost {
                min_cost = cost;
                continue;
            }

            if cost > to_visit[row][col] || cost >= min_cost {
                continue;
            }

            for dir in Direction::ALL {
                let next_dir = dir;
                let (next_row, next_col) = position + next_dir;
                let mut next_cost = cost;

                if next_dir == direction {
                    next_cost += 1;
                } else {
                    next_dir.rotate();
                    next_cost += 1001;
                }

                if self.grid[next_row][next_col] == Map::Wall {
                    continue;
                }

                if next_cost < to_visit[next_row][next_col] {
                    to_visit[next_row][next_col] = next_cost;

                    queue.push(Reverse(Tile {
                        position: (next_row, next_col),
                        direction: next_dir,
                        cost: next_cost,
                        history: None,
                    }));
                }
            }
        }

        min_cost
    }

    fn dijkstra_with_backtrack(&self, min_cost: usize) -> usize {
        let mut to_visit = vec![vec![[min_cost, min_cost]; self.width]; self.height];

        let mut queue = BinaryHeap::new();
        let mut tiles = HashSet::new();

        to_visit[self.start.0][self.start.1][Direction::get_axis(&Direction::East)] = 0;

        queue.push(Reverse(Tile {
            position: self.start,
            direction: Direction::East,
            cost: 0,
            history: Some(vec![]),
        }));

        while let Some(Reverse(Tile {
            position,
            direction,
            cost,
            history,
        })) = queue.pop()
        {
            let mut history = history.unwrap();
            history.push(position);

            let (row, col) = position;
            if cost > to_visit[row][col][direction.get_axis()] || cost > min_cost {
                continue;
            }

            if position == self.end {
                if cost == min_cost {
                    tiles.extend(history);
                }

                continue;
            }

            for dir in Direction::ALL {
                let next_dir = dir;
                let (next_row, next_col) = position + next_dir;
                let mut next_cost = cost;

                if next_dir == direction {
                    next_cost += 1;
                } else {
                    next_dir.rotate();
                    next_cost += 1001;
                }

                if self.grid[next_row][next_col] == Map::Wall {
                    continue;
                }

                if next_cost <= to_visit[next_row][next_col][direction.get_axis()] {
                    to_visit[next_row][next_col][direction.get_axis()] = next_cost;

                    queue.push(Reverse(Tile {
                        position: (next_row, next_col),
                        direction: next_dir,
                        cost: next_cost,
                        history: Some(history.clone()),
                    }));
                }
            }
        }

        tiles.len()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    North,
    East,
    West,
    South,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    fn rotate(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn get_axis(&self) -> usize {
        match self {
            Self::North | Self::South => 0,
            Self::West | Self::East => 1,
        }
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        let (row, col) = self;
        match rhs {
            Direction::North => (row - 1, col),
            Direction::East => (row, col + 1),
            Direction::South => (row + 1, col),
            Direction::West => (row, col - 1),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Map {
    Empty,
    Wall,
    Start,
    End,
}

impl From<char> for Map {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!("Please remove {} from your map", value),
        }
    }
}

impl From<&Map> for char {
    fn from(value: &Map) -> Self {
        match value {
            Map::Empty => '.',
            Map::Wall => '#',
            Map::Start => 'S',
            Map::End => 'E',
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Tile {
    position: (usize, usize),
    direction: Direction,
    cost: usize,
    history: Option<Vec<(usize, usize)>>,
}

pub fn part2() -> usize {
    let input = include_str!("/home/pragyan/aoc/2024/inputs/16");
    let maze = Maze::from(input);
    let min_cost = maze.dijkstra();
    maze.dijkstra_with_backtrack(min_cost)
}
