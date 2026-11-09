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
     let mut grid: &mut Grid = &mut new_grid();
print_grid(&grid);
grid[5][5] = true;
grid[6][5] = true;
grid[6][6] = true;

print_grid(&grid);
print_neighbor_counts(&grid);}
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
