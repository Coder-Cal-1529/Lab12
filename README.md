# Game Collection

A menu of text games, built one lab at a time.

## Week 12: Game of Life I

Menu option 6 runs `game_of_life::play()`. This week it shows a hand-set grid and a
neighbor-count view. It does not run generations yet.

Functions to write in `src/game_of_life.rs`: `new_grid`, `print_grid`, `count_neighbors`,
`next_state`, `print_neighbor_counts`, `play`.

Run: `cargo run`. Test: `cargo test`.

Rules: a live cell with 2 or 3 live neighbors stays alive; a dead cell with exactly 3 live
neighbors becomes alive; every other cell is dead in the next generation. The grid does not
wrap; cells beyond the edge count as dead.
