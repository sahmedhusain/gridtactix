use crate::types::{Grid, Piece, Player, Position};

pub fn find_opponent_center(grid: &Grid, player: &Player) -> (f64, f64) {
    let mut sum_x: f64 = 0.0;
    let mut sum_y: f64 = 0.0;
    let mut count: f64 = 0.0;

    for row in 0..grid.height {
        for col in 0..grid.width {
            if player.is_opponent(grid.cells[row][col]) {
                sum_x += col as f64;
                sum_y += row as f64;
                count += 1.0;
            }
        }
    }

    if count == 0.0 {
        (grid.width as f64 / 2.0, grid.height as f64 / 2.0)
    } else {
        (sum_x / count, sum_y / count)
    }
}

pub fn choose_best(
    positions: &[Position],
    piece: &Piece,
    grid: &Grid,
    player: &Player,
) -> Position {
    let (opp_cx, opp_cy) = find_opponent_center(grid, player);
    let filled = piece.filled_cells();

    let mut best_pos = positions[0];
    let mut best_distance = f64::MAX;

    for pos in positions {
        let mut piece_sum_x: f64 = 0.0;
        let mut piece_sum_y: f64 = 0.0;
        let count = filled.len() as f64;

        for &(px, py) in &filled {
            piece_sum_x += (pos.x + px as i32) as f64;
            piece_sum_y += (pos.y + py as i32) as f64;
        }

        let piece_cx = piece_sum_x / count;
        let piece_cy = piece_sum_y / count;

        let dx = piece_cx - opp_cx;
        let dy = piece_cy - opp_cy;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < best_distance {
            best_distance = distance;
            best_pos = *pos;
        }
    }

    best_pos
}
