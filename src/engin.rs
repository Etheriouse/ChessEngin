// all things about manipulation of board

use core::fmt;
use std::vec;

use crate::chess::{Piece, Pos, SIZE, Type};

pub struct Engin {
    board: [Option<Piece>; (SIZE * SIZE) as usize],
    played: bool,
}

/**
 * Movement
 */
impl Engin {
    pub fn moves(&self, from: &Pos) -> Vec<Pos> {
        return self.theoretical_moves(from);
    }

    fn theoretical_moves(&self, from: &Pos) -> Vec<Pos> {
        let mut base = vec![];
        let rook = vec![[1, 0], [-1, 0], [0, 1], [0, -1]];
        let bishop = vec![[1, 1], [1, -1], [-1, 1], [-1, -1]];
        let queen = vec![
            [1, 1],
            [1, -1],
            [-1, 1],
            [-1, -1],
            [1, 0],
            [-1, 0],
            [0, 1],
            [0, -1],
        ];
        let knight = vec![
            [2, 1],
            [2, -1],
            [-2, 1],
            [-2, -1],
            [1, 2],
            [1, -2],
            [-1, 2],
            [-1, -2],
        ];

        match self.board[from.index()] {
            None => (),
            Some(Piece { color, kind }) => match kind {
                Type::Rook => self.move_line(from, &mut base, color, &rook),
                Type::Bishop => self.move_line(from, &mut base, color, &bishop),
                Type::Knight => self.patern(from, &mut base, color, &knight),
                Type::King => (), //self.patern(from, &mut base, color, &knight),
                Type::Queen => self.move_line(from, &mut base, color, &queen),
                Type::Pawn => self.pawn(from, &mut base, color),
            },
        }

        return base;
    }

    fn move_line(&self, from: &Pos, base: &mut Vec<Pos>, color_piece: bool, dir: &Vec<[i8; 2]>) {
        for d in dir {
            let mut look = from.clone();
            while look.i < 8 && look.j < 8 && look.i >= 0 && look.j >= 0 {
                match self.board[look.index()] {
                    Some(Piece { color, kind: _ }) => {
                        if color ^ color_piece {
                            // adverse color
                            base.push(look.clone());
                        }
                        break;
                    }
                    None => base.push(look.clone()),
                }
                look.i += d[0];
                look.j += d[1];
            }
        }
    }

    fn patern(&self, from: &Pos, base: &mut Vec<Pos>, color_piece: bool, dir: &Vec<[i8; 2]>) {
        for d in dir {
            let i: i8 = from.i + d[0];
            let j: i8 = from.j + d[1];
            if !(i < 8 && j < 8 && i >= 0 && j >= 0) {
                continue;
            }
            match self.board[(i * 8 + j) as usize] {
                Some(Piece { color, kind: _ }) => {
                    if color ^ color_piece {
                        base.push(Pos::is(i, j));
                        continue;
                    }
                }
                None => base.push(Pos::is(i, j)),
            }
        }
    }

    fn pawn(&self, from: &Pos, base: &mut Vec<Pos>, color_piece: bool) {
        // black en letter 8 et white en letter 1

        let dir = if color_piece { -1 } else { 1 }; // vertical movement (i axis)
        let start = if color_piece { 6 } else { 1 }; // row 

        if from.j > 1 {
            match self.board[from.indexa(dir, -1)] {
                None => (),
                Some(piece) => {
                    if piece.color ^ color_piece {
                        base.push(from.from(dir, -1));
                    }
                }
            }
        }

        if from.j < SIZE - 2 {
            match self.board[from.indexa(dir, 1)] {
                None => (),
                Some(piece) => {
                    if piece.color ^ color_piece {
                        base.push(from.from(dir, 1));
                    }
                }
            }
        }

        let look = from.i + dir;
        if (0..SIZE).contains(&look) {
            if self.board[from.indexa(dir, 0)].is_none() {
                base.push(from.from(dir, 0));
                let look = from.i + dir * 2;
                if (0..SIZE).contains(&look)
                    && from.i == start
                    && self.board[from.indexa(dir * 2, 0)].is_none()
                {
                    base.push(from.from(dir * 2, 0));
                }
            }
        }
    }
}

/**
 * Manipulation
 */

impl Engin {
    fn set(&mut self, p: Piece, to: &Pos) {
        self.board[to.index()] = Some(p);
    }
}

/**
 * Constructor
 */

impl Engin {
    pub fn empty(played: bool) -> Self {
        Self {
            board: [None; (SIZE * SIZE) as usize],
            played,
        }
    }

    pub fn classic(played: bool) -> Self {
        let mut base = [None; (SIZE * SIZE) as usize];
        let white = [
            Some(Piece::new(true, Type::Rook)),
            Some(Piece::new(true, Type::Knight)),
            Some(Piece::new(true, Type::Bishop)),
            Some(Piece::new(true, Type::King)),
            Some(Piece::new(true, Type::Queen)),
            Some(Piece::new(true, Type::Bishop)),
            Some(Piece::new(true, Type::Knight)),
            Some(Piece::new(true, Type::Rook)),
        ];
        let black = [
            Some(Piece::new(false, Type::Rook)),
            Some(Piece::new(false, Type::Knight)),
            Some(Piece::new(false, Type::Bishop)),
            Some(Piece::new(false, Type::King)),
            Some(Piece::new(false, Type::Queen)),
            Some(Piece::new(false, Type::Bishop)),
            Some(Piece::new(false, Type::Knight)),
            Some(Piece::new(false, Type::Rook)),
        ];
        base[0..8].copy_from_slice(&black);
        base[8..16].copy_from_slice(&[Some(Piece::new(false, Type::Pawn)); SIZE as usize]);
        base[48..56].copy_from_slice(&[Some(Piece::new(true, Type::Pawn)); SIZE as usize]);
        base[56..64].copy_from_slice(&white);

        Self {
            board: base,
            played,
        }
    }
}

/**
 * Pretty print
 */
impl fmt::Display for Engin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..SIZE {
            let row = if self.played { i } else { SIZE - 1 - i };
            write!(f, "{}", SIZE - row)?;

            for j in 0..SIZE {
                let col = if self.played { j } else { SIZE - 1 - j };

                let index = (row * SIZE + col) as usize;

                match self.board[index] {
                    Some(p) => write!(f, " {}", p.icon())?,
                    None => write!(f, "  .")?,
                }
            }
            writeln!(f, "")?;
        }

        for j in 0..SIZE {
            let col = if self.played { j } else { SIZE - 1 - j };
            write!(f, "  {}", (b'a' + col as u8) as char)?;
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "test/tests.rs"]
mod tests;
