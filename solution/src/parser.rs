use std::io::{self, BufRead};
use crate::types::{Grid, Piece, Player};

pub fn parse_player(line: &str) -> Player {
    if line.contains("p1") {
        Player::P1
    } else {
        Player::P2
    }
}

pub fn parse_grid(lines: &[String]) -> Grid {
    let header = &lines[0];
    let parts: Vec<&str> = header.split_whitespace().collect();
    let width: usize = parts[1].parse().expect("Failed to parse grid width");
    let height: usize = parts[2].trim_end_matches(':').parse().expect("Failed to parse grid height");

    let mut cells = Vec::with_capacity(height);
    for i in 0..height {
        let row_line = &lines[2 + i];
        let content: String = if row_line.len() > 4 {
            row_line[4..].chars().collect()
        } else {
            String::new()
        };
        let row: Vec<char> = content.chars().collect();
        cells.push(row);
    }

    Grid { width, height, cells }
}

pub fn parse_piece(lines: &[String]) -> Piece {
    let header = &lines[0];
    let parts: Vec<&str> = header.split_whitespace().collect();
    let width: usize = parts[1].parse().expect("Failed to parse piece width");
    let height: usize = parts[2].trim_end_matches(':').parse().expect("Failed to parse piece height");

    let mut cells = Vec::with_capacity(height);
    for i in 0..height {
        let row_line = &lines[1 + i];
        let row: Vec<bool> = row_line.chars().map(|c| c == 'O' || c == '*').collect();
        cells.push(row);
    }

    Piece { width, height, cells }
}

pub fn read_turn(reader: &mut dyn BufRead) -> io::Result<(Grid, Piece)> {
    let mut grid_lines: Vec<String> = Vec::new();
    let mut piece_lines: Vec<String> = Vec::new();
    let mut reading_grid = false;
    let mut reading_piece = false;
    let mut grid_rows_remaining: usize = 0;
    let mut piece_rows_remaining: usize = 0;
    let mut skip_column_header = false;

    loop {
        let mut line = String::new();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Game over"));
        }
        let line = line.trim_end_matches('\n').trim_end_matches('\r').to_string();

        if line.starts_with("Anfield") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let height: usize = parts[2].trim_end_matches(':').parse().unwrap_or(0);
            grid_lines.push(line);
            reading_grid = true;
            reading_piece = false;
            grid_rows_remaining = height;
            skip_column_header = true;
            continue;
        }

        if line.starts_with("Piece") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let height: usize = parts[2].trim_end_matches(':').parse().unwrap_or(0);
            piece_lines.push(line);
            reading_grid = false;
            reading_piece = true;
            piece_rows_remaining = height;
            continue;
        }

        if reading_grid {
            if skip_column_header {
                grid_lines.push(line);
                skip_column_header = false;
                continue;
            }
            grid_lines.push(line);
            grid_rows_remaining -= 1;
            if grid_rows_remaining == 0 {
                reading_grid = false;
            }
            continue;
        }

        if reading_piece {
            piece_lines.push(line);
            piece_rows_remaining -= 1;
            if piece_rows_remaining == 0 {
                break;
            }
            continue;
        }

    }

    let grid = parse_grid(&grid_lines);
    let piece = parse_piece(&piece_lines);

    Ok((grid, piece))
}

pub fn read_player(reader: &mut dyn BufRead) -> io::Result<Player> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    Ok(parse_player(&line))
}
