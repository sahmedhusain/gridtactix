use filler::types::{Grid, Piece, Player, Position};
use filler::parser::{parse_player, parse_grid, parse_piece};
use filler::placement::{is_valid_placement, find_valid_placements};
use filler::strategy::{find_opponent_center, choose_best};

// INPUT PARSING TESTS

#[test]
fn test_parse_player_p1() {
    let line = "$$$ exec p1 : [robots/bender]";
    let player = parse_player(line);
    assert_eq!(player, Player::P1);
}

#[test]
fn test_parse_player_p2() {
    let line = "$$$ exec p2 : [robots/terminator]";
    let player = parse_player(line);
    assert_eq!(player, Player::P2);
}

#[test]
fn test_parse_grid_dimensions() {
    let lines: Vec<String> = vec![
        "Anfield 10 3:".to_string(),
        "    0123456789".to_string(),
        "000 ..........".to_string(),
        "001 ....@.....".to_string(),
        "002 ..........".to_string(),
    ];
    let grid = parse_grid(&lines);
    assert_eq!(grid.width, 10);
    assert_eq!(grid.height, 3);
}

#[test]
fn test_parse_grid_cells() {
    let lines: Vec<String> = vec![
        "Anfield 10 3:".to_string(),
        "    0123456789".to_string(),
        "000 ..........".to_string(),
        "001 ....@.....".to_string(),
        "002 .......$..".to_string(),
    ];
    let grid = parse_grid(&lines);
    // '@' should be at row 1, col 4
    assert_eq!(grid.cells[1][4], '@');
    // '$' should be at row 2, col 7
    assert_eq!(grid.cells[2][7], '$');
    // Empty cell
    assert_eq!(grid.cells[0][0], '.');
}

#[test]
fn test_parse_piece_dimensions() {
    let lines: Vec<String> = vec![
        "Piece 4 2:".to_string(),
        ".OO.".to_string(),
        "..O.".to_string(),
    ];
    let piece = parse_piece(&lines);
    assert_eq!(piece.width, 4);
    assert_eq!(piece.height, 2);
}

#[test]
fn test_parse_piece_cells() {
    let lines: Vec<String> = vec![
        "Piece 4 2:".to_string(),
        ".OO.".to_string(),
        "..O.".to_string(),
    ];
    let piece = parse_piece(&lines);
    // Row 0: .OO. → false, true, true, false
    assert!(!piece.cells[0][0]);
    assert!(piece.cells[0][1]);
    assert!(piece.cells[0][2]);
    assert!(!piece.cells[0][3]);
    // Row 1: ..O. → false, false, true, false
    assert!(!piece.cells[1][0]);
    assert!(!piece.cells[1][1]);
    assert!(piece.cells[1][2]);
    assert!(!piece.cells[1][3]);
}

#[test]
fn test_parse_piece_star_notation() {
    // Some game engines use '*' instead of 'O'
    let lines: Vec<String> = vec![
        "Piece 3 1:".to_string(),
        "**.".to_string(),
    ];
    let piece = parse_piece(&lines);
    assert!(piece.cells[0][0]);
    assert!(piece.cells[0][1]);
    assert!(!piece.cells[0][2]);
}

#[test]
fn test_piece_filled_cells() {
    let piece = Piece {
        width: 3,
        height: 2,
        cells: vec![
            vec![false, true, false],
            vec![true, true, false],
        ],
    };
    let filled = piece.filled_cells();
    // Should be (1,0), (0,1), (1,1)
    assert_eq!(filled.len(), 3);
    assert!(filled.contains(&(1, 0)));
    assert!(filled.contains(&(0, 1)));
    assert!(filled.contains(&(1, 1)));
}

// PLACEMENT VALIDATION TESTS

fn make_grid(rows: &[&str]) -> Grid {
    let height = rows.len();
    let width = rows[0].len();
    let cells: Vec<Vec<char>> = rows.iter().map(|r| r.chars().collect()).collect();
    Grid { width, height, cells }
}

fn make_piece(rows: &[&str]) -> Piece {
    let height = rows.len();
    let width = rows[0].len();
    let cells: Vec<Vec<bool>> = rows.iter().map(|r| r.chars().map(|c| c == 'O' || c == '*').collect()).collect();
    Piece { width, height, cells }
}

#[test]
fn test_valid_placement_one_overlap() {
    let grid = make_grid(&[
        ".....",
        "..@..",
        ".....",
    ]);
    // Piece: single cell "O"
    let piece = make_piece(&["O"]);
    let player = Player::P1;

    // Placing at (2, 1) — directly on top of our '@' → exactly 1 overlap → VALID
    assert!(is_valid_placement(&grid, &piece, &player, 2, 1));
}

#[test]
fn test_invalid_placement_no_overlap() {
    let grid = make_grid(&[
        ".....",
        "..@..",
        ".....",
    ]);
    let piece = make_piece(&["O"]);
    let player = Player::P1;

    // Placing at (0, 0) — no overlap with our territory → INVALID
    assert!(!is_valid_placement(&grid, &piece, &player, 0, 0));
}

#[test]
fn test_invalid_placement_two_overlaps_with_own() {
    // Grid with two of our cells adjacent
    let grid = make_grid(&[
        ".....",
        "..@@.",
        ".....",
    ]);
    // Piece: two horizontal cells "OO"
    let piece = make_piece(&["OO"]);
    let player = Player::P1;

    // Placing at (2, 1) → both cells land on '@' → 2 overlaps → INVALID
    assert!(!is_valid_placement(&grid, &piece, &player, 2, 1));
}

#[test]
fn test_invalid_placement_overlap_with_opponent() {
    let grid = make_grid(&[
        ".....",
        "..@$.",
        ".....",
    ]);
    let piece = make_piece(&["OO"]);
    let player = Player::P1;

    // Placing at (2, 1) → one on '@' (ours), one on '$' (opponent) → INVALID
    assert!(!is_valid_placement(&grid, &piece, &player, 2, 1));
}

#[test]
fn test_valid_placement_adjacent_to_own() {
    let grid = make_grid(&[
        ".....",
        "..@..",
        ".....",
    ]);
    let piece = make_piece(&["OO"]);
    let player = Player::P1;

    // Placing at (1, 1) → first cell on '.', second on '@' → exactly 1 overlap → VALID
    assert!(is_valid_placement(&grid, &piece, &player, 1, 1));

    // Placing at (2, 1) → first cell on '@', second on '.' → exactly 1 overlap → VALID
    assert!(is_valid_placement(&grid, &piece, &player, 2, 1));
}

#[test]
fn test_find_valid_placements_count() {
    let grid = make_grid(&[
        ".....",
        "..@..",
        ".....",
    ]);
    let piece = make_piece(&["O"]);
    let player = Player::P1;

    // A single 'O' piece can be placed on exactly 1 position: on top of '@'
    let valids = find_valid_placements(&grid, &piece, &player);
    assert_eq!(valids.len(), 1);
    assert_eq!(valids[0], Position { x: 2, y: 1 });
}

// BOUNDARY DETECTION TESTS

#[test]
fn test_placement_out_of_bounds_right() {
    let grid = make_grid(&[
        "...@.",  // '@' at col 3
    ]);
    // Piece: 3 cells wide "OOO"
    let piece = make_piece(&["OOO"]);
    let player = Player::P1;

    // Placing at (3, 0) → cells at col 3,4,5 → col 5 is out of bounds (width=5) → INVALID
    assert!(!is_valid_placement(&grid, &piece, &player, 3, 0));
}

#[test]
fn test_placement_out_of_bounds_bottom() {
    let grid = make_grid(&[
        ".....",
        "..@..",
    ]);
    let piece = make_piece(&["O", "O"]); // 2 rows tall
    let player = Player::P1;

    // Placing at (2, 1) → rows 1 and 2 → row 2 is out of bounds (height=2) → INVALID
    assert!(!is_valid_placement(&grid, &piece, &player, 2, 1));
}

#[test]
fn test_placement_out_of_bounds_left() {
    let grid = make_grid(&[
        "@....",
        ".....",
    ]);
    let piece = make_piece(&["OO"]);
    let player = Player::P1;

    // Placing at (-1, 0) → cell at col -1 is out of bounds → INVALID
    assert!(!is_valid_placement(&grid, &piece, &player, -1, 0));
}

#[test]
fn test_placement_out_of_bounds_top() {
    let grid = make_grid(&[
        "@....",
        ".....",
    ]);
    let piece = make_piece(&["O", "O"]);
    let player = Player::P1;

    // Placing at (0, -1) → row -1 is out of bounds → INVALID
    assert!(!is_valid_placement(&grid, &piece, &player, 0, -1));
}

#[test]
fn test_placement_negative_offset_valid() {
    // Piece has empty cells in its top-left, so negative offsets CAN be valid
    let grid = make_grid(&[
        "@....",
        ".....",
    ]);
    // Piece: ".O" — the empty cell hangs off the left edge, but the filled cell 'O' is at col 0
    let piece = make_piece(&[".O"]);
    let player = Player::P1;

    // Placing at (-1, 0) → the '.' cell is at col -1 (ignored since it's empty),
    // the 'O' cell is at col 0 → on '@' → 1 overlap → VALID
    assert!(is_valid_placement(&grid, &piece, &player, -1, 0));
}

// COORDINATE OUTPUT FORMAT TESTS

#[test]
fn test_coordinate_output_format() {
    let pos = Position { x: 7, y: 2 };
    let output = format!("{} {}", pos.x, pos.y);
    assert_eq!(output, "7 2");
}

#[test]
fn test_coordinate_output_format_negative() {
    let pos = Position { x: -1, y: 3 };
    let output = format!("{} {}", pos.x, pos.y);
    assert_eq!(output, "-1 3");
}

#[test]
fn test_default_output_when_no_valid_moves() {
    let grid = make_grid(&[
        "$$$$$",
        "$@$$$",
        "$$$$$",
    ]);
    let piece = make_piece(&["OO"]);
    let player = Player::P1;

    // '@' is surrounded by opponent cells — no valid placement possible
    let valids = find_valid_placements(&grid, &piece, &player);
    assert!(valids.is_empty());

    // In this case, the robot should output "0 0"
    let output = if valids.is_empty() {
        "0 0".to_string()
    } else {
        let best = &valids[0];
        format!("{} {}", best.x, best.y)
    };
    assert_eq!(output, "0 0");
}

// STRATEGY TESTS

#[test]
fn test_opponent_center() {
    let grid = make_grid(&[
        ".....",
        ".....",
        "...$.",  // opponent at (3, 2)
        ".....",
    ]);
    let player = Player::P1;
    let (cx, cy) = find_opponent_center(&grid, &player);
    assert_eq!(cx, 3.0);
    assert_eq!(cy, 2.0);
}

#[test]
fn test_opponent_center_multiple_cells() {
    let grid = make_grid(&[
        ".....",
        ".$...",  // opponent at (1, 1)
        "...$.",  // opponent at (3, 2)
        ".....",
    ]);
    let player = Player::P1;
    let (cx, cy) = find_opponent_center(&grid, &player);
    // Average: x = (1 + 3) / 2 = 2.0, y = (1 + 2) / 2 = 1.5
    assert_eq!(cx, 2.0);
    assert_eq!(cy, 1.5);
}

#[test]
fn test_choose_best_moves_toward_opponent() {
    // Grid: our cell at (0, 0), opponent at (4, 0)
    let grid = make_grid(&[
        "@...$",
    ]);
    let piece = make_piece(&["O"]);
    let player = Player::P1;

    let valids = find_valid_placements(&grid, &piece, &player);
    // The only valid placement is on '@' itself (overlaps 1 own cell)
    // With more cells we'd see the strategy pick the closest to opponent
    let best = choose_best(&valids, &piece, &grid, &player);
    // Should be (0, 0) — the only valid position
    assert_eq!(best.x, 0);
    assert_eq!(best.y, 0);
}

#[test]
fn test_player_p2_chars() {
    let player = Player::P2;
    assert!(player.is_mine('$'));
    assert!(player.is_mine('s'));
    assert!(!player.is_mine('@'));
    assert!(!player.is_mine('a'));
    assert!(player.is_opponent('@'));
    assert!(player.is_opponent('a'));
}

#[test]
fn test_player_p1_chars() {
    let player = Player::P1;
    assert!(player.is_mine('@'));
    assert!(player.is_mine('a'));
    assert!(!player.is_mine('$'));
    assert!(player.is_opponent('$'));
    assert!(player.is_opponent('s'));
}
