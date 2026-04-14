use crate::types::{Grid, Piece, Player, Position};

pub fn is_valid_placement(grid: &Grid, piece: &Piece, player: &Player, x: i32, y: i32) -> bool {
    let mut overlap_mine = 0;
    let mut overlap_opponent = 0;

    for &(px, py) in &piece.filled_cells() {
        let actual_x = x + px as i32;
        let actual_y = y + py as i32;

        if actual_x < 0
            || actual_y < 0
            || actual_x >= grid.width as i32
            || actual_y >= grid.height as i32
        {
            return false; //invalid
        }

        let cell = grid.cells[actual_y as usize][actual_x as usize];

        if player.is_mine(cell) {
            overlap_mine += 1;
        }
        if player.is_opponent(cell) {
            overlap_opponent += 1;
        }
    }

    overlap_mine == 1 && overlap_opponent == 0
}

pub fn find_valid_placements(grid: &Grid, piece: &Piece, player: &Player) -> Vec<Position> {
    let mut valid_positions = Vec::new();

    let min_y = -(piece.height as i32 - 1);
    let max_y = grid.height as i32;
    let min_x = -(piece.width as i32 - 1);
    let max_x = grid.width as i32;

    for y in min_y..max_y {
        for x in min_x..max_x {
            if is_valid_placement(grid, piece, player, x, y) {
                valid_positions.push(Position { x, y });
            }
        }
    }

    valid_positions
}
