// Game of Life, Week 12: grid, display, neighbor counts, the rule for one cell.
//
// TODO: remove the next line once every function below is used.

use crate::input::{read_line_trimmed};
const ROWS: usize = 10;
const COLS: usize = 20;
type Grid = [[bool; COLS]; ROWS];

// All cells dead.
pub fn new_grid() -> Grid {
    let mut grid = [[false; COLS]; ROWS];
    for r in 0..ROWS{
	for c in 0..COLS{
		print!(". ");
		grid[r][c] = false;
	}
    	println!(" ");
    }
grid
}

// '#' for a live cell, '.' for a dead cell, one line per row.
pub fn print_grid(grid: &Grid) {
    for r in 0..ROWS{
        for c in 0..COLS{
            match grid[r][c]{
		    true => print!("# "),
                    false => print!(". "),
	    }
        }
        println!(" ");
    }
}

// Live neighbors of (row, col): the eight surrounding cells, excluding the cell itself.
// Cells beyond the edge count as dead. Careful: row - 1 on a usize of 0 panics.
pub fn count_neighbors(grid: &Grid, row: usize, col: usize) -> u8 {
    let mut count = 0;
    let directions = [ (-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];
    for &(dr, dc) in &directions{
	let r = row as i32 + dr;
	let c = col as i32 +dc;
	if r >= 0 && r <ROWS as i32 && c>= 0 && c < COLS as i32{
	    if grid[r as usize][c as usize] {
	         count+=1;  
	    }
	}	
    }
    count
}
// The rule for one cell: does it live in the next generation?
pub fn next_state(alive: bool, neighbors: u8) -> bool { 
    
	match (alive,neighbors){
	(true,2|3) => true,
	(false,3) => true,
	(_,_) => false,
    }
}

// Debug view: each cell's neighbor count as a digit, one line per row.
pub fn print_neighbor_counts(grid: &Grid) {
    for r in 0..ROWS{
	for c in 0..COLS{
	    print!("{} ", count_neighbors(grid,r,c));
	}println!(" ") 
    }
}

// Set a few cells alive (hardcoded, or entered as `row col` pairs until a blank line),
// print the grid, then the neighbor counts.
pub fn play() {
    let mut grid: Grid = new_grid();
    let mut input_count: u32 = 0;
       while input_count < 5 {
        println!("Choose starting live cell placement: type something like '2,4' or '2 4' (up to {}, {})", ROWS, COLS);
        println!("{} of 5 cells placed.", input_count);
        let input = read_line_trimmed();
 let mut input_check = None;
        for (i, ch) in input.chars().enumerate() {
            if ch == ',' || ch == ' ' || ch == '\t' {
                input_check = Some(i);
                break;
            }
        }
        let clean_input = match input_check {
            Some(i) => i,
            None => {
                println!("Please input exactly 2 whole numbers in the requested format (e.g., 2,4).");
                continue;
            }
        };
        let row_part = input[..clean_input].trim();
        let col_part = input[clean_input + 1..].trim();
        let row: i32 = row_part.parse().unwrap_or(0);
        let col: i32 = col_part.parse().unwrap_or(0);

        if row < 1 || row > ROWS as i32 || col < 1 || col > COLS as i32 {
            println!("Input out of bounds, Rows must be 1-{} and Cols must be 1-{}.", ROWS, COLS);
        } else {
            grid[(row - 1) as usize][(col - 1) as usize] = true;
            input_count += 1;
        }
    }
println!("--- Game Start ---");
    print_grid(&grid);
    print_neighbor_counts(&grid);
loop {
        print!("(c)ontinue or (e)nd game? ");
        std::io::Write::flush(&mut std::io::stdout()).expect("failed to flush");       
        let answer = read_line_trimmed();
        if answer == "e" {
            println!("");
            break;
        } else if answer == "c" {
let mut next_grid = new_grid();
            for r in 0..ROWS {
                for c in 0..COLS {
                    next_grid[r][c] = next_state(grid[r][c], count_neighbors(&grid, r, c));
                }
            }
            grid = next_grid;
            println!("--- Next Generation ---");
            print_grid(&grid);
            print_neighbor_counts(&grid);
        } else {
            println!("Invalid choice. Please type 'c' or 'e'.");
       }      
}
}
#[cfg(test)]
mod tests {
   use super::*;

    // Supplied by the instructor.
    #[test]
    fn live_1_dies() {
        assert!(!next_state(true, 1));
    }

    #[test]
    fn live_2_survives() {
        assert!(next_state(true, 2));
    }

    #[test]
    fn dead_3_born() {
        assert!(next_state(false, 3));
    }
    #[test]
    fn live_4_dies() {
	assert!(!next_state(true, 4));
}
    #[test]
    fn dead_2_stays_dead() {
	assert!(!next_state(false,2));
}
    // TODO (student): write the remaining cases from your truth table before you
    // write next_state. At minimum: a live cell with 4 neighbors, a dead cell with 2.
}
