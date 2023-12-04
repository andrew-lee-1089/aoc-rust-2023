use crate::utils::read_lines;

fn get_nos_in_row(row: &Vec<char>, pivot_index: usize) -> Vec<u32> {
    let mut surrounding_nos: Vec<u32> = vec![];
    if row[pivot_index].is_ascii_digit() && surrounding_nos.len() == 0 {
        surrounding_nos.push(find_rest_of_number(row, pivot_index));
        return surrounding_nos;
    }

    if (pivot_index != 0) && row[pivot_index - 1].is_ascii_digit() {
        surrounding_nos.push(find_rest_of_number(row, pivot_index - 1));
    }
    if pivot_index != row.len() && row[pivot_index + 1].is_ascii_digit() {
        surrounding_nos.push(find_rest_of_number(row, pivot_index + 1));
    }
    surrounding_nos
}

fn find_rest_of_number(row: &Vec<char>, index: usize) -> u32 {
    let mut number_chars: Vec<char> = vec![];
    let mut search_idx: usize = index;

    while search_idx < row.len() && row[search_idx].is_ascii_digit() {
        number_chars.push(row[search_idx]);
        search_idx += 1;
    }
    
    search_idx = index - 1;
    while search_idx >= 0 && row[search_idx].is_ascii_digit() {
        number_chars.insert(0, row[search_idx]);
        search_idx -= 1;
    }

    number_chars
        .iter()
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

struct MachinePart {
    machine_type: char,
    surrounding_nos: Vec<u32>,
}

impl MachinePart {
    fn sum_of_surrouding_parts(&self) -> u32 {
        self.surrounding_nos.iter().sum()
    }

    fn from_grid(grid: &Vec<Vec<char>>, row_id: usize, col_id: usize) -> MachinePart {
        let machine_type = grid[row_id][col_id];
        let mut surrounding_nos: Vec<u32> = vec![];
        if row_id != 0 {
            surrounding_nos.extend(get_nos_in_row(&grid[row_id - 1], col_id));
        }
        surrounding_nos.extend(get_nos_in_row(&grid[row_id], col_id));
        if row_id != grid.len() {
            surrounding_nos.extend(get_nos_in_row(&grid[row_id + 1], col_id))
        }

        MachinePart {
            machine_type,
            surrounding_nos,
        }
    }
}

fn parse() -> Vec<MachinePart> {
    let inputs = read_lines("inputs/day03.txt");
    let chars_vec: Vec<Vec<char>> = inputs.iter().map(|line| line.chars().collect()).collect();

    // We assume in solving that each number is adjacent to at most one
    // machine part, this motivates an algorithm to:
    // (a) find all the machine parts (non-digit, non-period, characters)
    // (b) find all numbers attached to each machine part
    let mut machine_parts: Vec<MachinePart> = vec![];

    for (id_y, row) in chars_vec.iter().enumerate() {
        for (id_x, cell) in row.iter().enumerate() {
            if cell != &'.' && !cell.is_digit(10) {
                machine_parts.push(MachinePart::from_grid(&chars_vec, id_y, id_x))
            }
        }
    }
    machine_parts
}

pub fn part1solve() -> u32 {
    let machine_parts = parse();
    machine_parts
        .iter()
        .map(|machine_part| machine_part.sum_of_surrouding_parts())
        .sum()
}

pub fn part2solve() -> u32 {
    let machine_parts = parse();
    machine_parts
        .iter()
        .filter(|machine_part| {
            machine_part.machine_type == '*' && machine_part.surrounding_nos.len() == 2
        })
        .map(|machine_part| machine_part.surrounding_nos[0] * machine_part.surrounding_nos[1])
        .sum()
}
