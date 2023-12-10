// TIL:
// Well that was horrid and hard a waste of about 3 hours.
// I'm not sure I learnt much, but I have aged many years

use std::{cmp::min, collections::HashMap};

use crate::utils::read_lines;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum CellType {
    Empty,
    Tube(Direction, Direction),
    StartingCell, // S
}

impl CellType {
    fn from_char(c: char) -> CellType {
        match c {
            '.' => CellType::Empty,
            '|' => CellType::Tube(Direction::North, Direction::South),
            'L' => CellType::Tube(Direction::North, Direction::East),
            'J' => CellType::Tube(Direction::North, Direction::West),
            '-' => CellType::Tube(Direction::West, Direction::East),
            '7' => CellType::Tube(Direction::West, Direction::South),
            'F' => CellType::Tube(Direction::South, Direction::East),
            'S' => CellType::StartingCell,
            _ => panic!("At the disco"),
        }
    }

    fn passes_west(&self) -> bool {
        match self {
            CellType::Empty => false,
            CellType::StartingCell => true, // empeircal fact :-(
            CellType::Tube(a, b) => a == &Direction::West || b == &Direction::West,
        }
    }

    fn passes_east(&self) -> bool {
        match self {
            CellType::Empty => false,
            CellType::StartingCell => false,
            CellType::Tube(a, b) => a == &Direction::East || b == &Direction::East,
        }
    }

    fn can_enter_from(&self, entering_direction: Direction, entering_cell: CellType) -> bool {
        // leaving entering_cell from entering_direction and going into self from opposite direction
        let can_leave_this_cell = match entering_cell {
            CellType::Empty => false,
            CellType::Tube(a, b) => a == entering_direction || b == entering_direction,
            CellType::StartingCell => true,
        };

        let can_enter_this_cell = match self {
            CellType::Empty => false,
            CellType::Tube(a, b) => {
                a == &Direction::opposite(&entering_direction)
                    || b == &Direction::opposite(&entering_direction)
            }
            CellType::StartingCell => true,
        };
        can_leave_this_cell && can_enter_this_cell
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match &self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

fn get_ortho_cells(
    coords: &(i32, i32),
    grid: &HashMap<(i32, i32), CellType>,
) -> HashMap<Direction, ((i32, i32), CellType)> {
    let mut ortho_cells: HashMap<Direction, ((i32, i32), CellType)> = HashMap::new();
    let (row, column_) = coords;
    if row != &0 {
        ortho_cells.insert(
            Direction::North,
            (
                (row - 1, *column_),
                grid.get(&(row - 1, *column_)).unwrap().clone(),
            ),
        );
    }
    if row != &139 {
        ortho_cells.insert(
            Direction::South,
            (
                (row + 1, *column_),
                grid.get(&(row + 1, *column_)).unwrap().clone(),
            ),
        );
    }
    if column_ != &0 {
        ortho_cells.insert(
            Direction::West,
            (
                (*row, column_ - 1),
                grid.get(&(*row, column_ - 1)).unwrap().clone(),
            ),
        );
    }
    if column_ != &139 {
        ortho_cells.insert(
            Direction::East,
            (
                (*row, column_ + 1),
                grid.get(&(*row, column_ + 1)).unwrap().clone(),
            ),
        );
    }
    ortho_cells
}

fn parse() -> HashMap<(i32, i32), CellType> {
    let inputs = read_lines("inputs/day10.txt");
    let mut grid: HashMap<(i32, i32), CellType> = HashMap::new();
    for row in 0..inputs.len() {
        for column_ in 0..inputs[0].len() {
            grid.insert(
                (row as i32, column_ as i32),
                CellType::from_char(inputs[row].chars().nth(column_).unwrap()),
            );
        }
    }
    grid
}

fn make_loop(grid: &HashMap<(i32, i32), CellType>) -> Vec<(i32, i32)> {
    let starting_coords: &(i32, i32) = grid
        .keys()
        .into_iter()
        .filter(|k| grid.get(k).unwrap() == &CellType::StartingCell)
        .next()
        .unwrap();

    let mut the_loop: Vec<(i32, i32)> = vec![];
    let mut boundaries: Vec<(Option<Direction>, (i32, i32), CellType)> =
        vec![(None, starting_coords.clone(), CellType::StartingCell)];

    while boundaries.len() > 0 {
        // Choose a boundaries in the list
        let (entering_direction, cell_coords, cell_type) = boundaries.pop().unwrap();

        // let 'find all the orthogoanl cells
        let ortho_cells = get_ortho_cells(&cell_coords, &grid);

        // let's say we entered cell foo from direction X
        // Loop through all the other directions, other than the one we came into and see if we can there, ofr the starting cell, we look in all directions
        for direction in ortho_cells.keys().into_iter() {
            if let Some(real_entrydirection) = entering_direction {
                if direction == &real_entrydirection {
                    continue;
                }
            }
            if CellType::can_enter_from(&ortho_cells[direction].1, direction.clone(), cell_type) {
                // if we can go this way, add the cell we can go to into the boundaries queue
                boundaries.push((
                    // if we go south to a cell, we add that came from the north
                    Some(Direction::opposite(direction)),
                    ortho_cells[direction].0.clone(),
                    ortho_cells[direction].1,
                ));
                if the_loop.contains(&cell_coords) {
                    // we've found the loop
                    the_loop.push(*starting_coords);
                    return the_loop;
                }
                if &cell_coords != starting_coords {
                    the_loop.push(cell_coords);
                }
            }
        }
    }
    the_loop
}

pub fn part1solve() -> i32 {
    let grid = parse();

    let the_loop: Vec<(i32, i32)> = make_loop(&grid);
    ((the_loop.len() + 1) / 2) as i32
}

pub fn part2solve() -> i32 {
    // We make use of the Jordan curve theorem - in other words, drawing a line from the interior to the edge will cross the loop an odd number of times
    let grid = parse();
    let the_loop: Vec<(i32, i32)> = make_loop(&grid);
    let mut count_of_interior = 0;

    for cell in grid.keys().clone().into_iter() {
        if the_loop.contains(cell) {
            continue;
        }
        // get all cells above it
        // that are in the loop
        // and count the nubmer of west-east instances
        let celltypes = (0..cell.0)
            .into_iter()
            .filter(|row| the_loop.contains(&(*row, cell.1)))
            .map(|row| grid.get(&(row, cell.1)).unwrap().clone())
            .collect::<Vec<CellType>>();
        let west_count = celltypes
            .iter()
            .filter(|ct: &&CellType| CellType::passes_west(ct))
            .count();
        let east_count = celltypes
            .iter()
            .filter(|ct: &&CellType| CellType::passes_east(ct))
            .count();

        if min(west_count, east_count) % 2 == 1 {
            println!("{:?} is in the interior!", cell);
            count_of_interior += 1
        }
    }
    count_of_interior
}
