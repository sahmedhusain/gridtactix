#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    P1,
    P2,
}

impl Player {
    pub fn my_chars(&self) -> (char, char) {
        match self {
            Player::P1 => ('@', 'a'),
            Player::P2 => ('$', 's'),
        }
    }

    pub fn opponent_chars(&self) -> (char, char) {
        match self {
            Player::P1 => ('$', 's'),
            Player::P2 => ('@', 'a'),
        }
    }

    pub fn is_mine(&self, c: char) -> bool {
        let (c1, c2) = self.my_chars();
        c == c1 || c == c2
    }

    pub fn is_opponent(&self, c: char) -> bool {
        let (c1, c2) = self.opponent_chars();
        c == c1 || c == c2
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32, // column
    pub y: i32, // row
}

#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<bool>>, // cells[row][col]
}

impl Piece {
    pub fn filled_cells(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for row in 0..self.height {
            for col in 0..self.width {
                if self.cells[row][col] {
                    result.push((col, row)); // (x, y)
                }
            }
        }
        result
    }
}

/// Game board
/// '.', '@', 'a', '$', or 's'
#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<char>>, // cells[row][col] character
}

impl Grid {
    pub fn get(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            None
        } else {
            Some(self.cells[y as usize][x as usize])
        }
    }
}
