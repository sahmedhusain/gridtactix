use crate::types::{Grid, Piece, Player, Position};

pub fn find_opponent_center(grid: &Grid, player: &Player) -> (f64, f64) {
    let cells = get_opponent_cells(grid, player);
    if cells.is_empty() {
        return (0.0, 0.0);
    }
    let sum_x: i32 = cells.iter().map(|&(x, _)| x).sum();
    let sum_y: i32 = cells.iter().map(|&(_, y)| y).sum();
    let n = cells.len() as f64;
    (sum_x as f64 / n, sum_y as f64 / n)
}

pub fn get_opponent_cells(grid: &Grid, player: &Player) -> Vec<(i32, i32)> {
    let mut cells = Vec::new();
    for row in 0..grid.height {
        for col in 0..grid.width {
            if player.is_opponent(grid.cells[row][col]) {
                cells.push((col as i32, row as i32));
            }
        }
    }
    cells
}

pub fn choose_best(
    positions: &[Position],
    piece: &Piece,
    grid: &Grid,
    player: &Player,
) -> Position {
    let opp_cells = get_opponent_cells(grid, player);
    let filled = piece.filled_cells();

    let mut best_pos = positions[0];
    let mut best_distance = f64::MAX;

    for pos in positions {
        let mut min_dist_to_opp = f64::MAX;

        for &(px, py) in &filled {
            let piece_x = pos.x + px as i32;
            let piece_y = pos.y + py as i32;

            for &(ox, oy) in &opp_cells {
                let dx = (piece_x - ox) as f64;
                let dy = (piece_y - oy) as f64;
                // Use squared distance for speed (no sqrt needed)
                let dist = dx * dx + dy * dy;
                if dist < min_dist_to_opp {
                    min_dist_to_opp = dist;
                }
            }
        }

        // We want to minimize the minimum distance (stick to the opponent)
        if min_dist_to_opp < best_distance {
            best_distance = min_dist_to_opp;
            best_pos = *pos;
        }
    }

    best_pos
}
