use core::fmt;
use std::{collections::HashMap, vec, write};

use crate::chess_lib::chess_lib::{Piece, PieceKind, Pos};

struct PlayerData {
    eat: HashMap<Piece, u8>,
    control: HashMap<Pos, u8>,
}

pub struct Engin {
    board: [Option<Piece>; 64],
    played: bool,

    white: PlayerData,
    black: PlayerData,

    history: Vec<(Pos, Pos)>,

    // position if pass is possible
    // position are one cell behind pawn
    pass: Option<Pos>,
}

// base
impl Engin {
    pub fn empty(played: bool) -> Self {
        Self {
            board: [None; 64],
            played,
            white: PlayerData {
                eat: HashMap::new(),
                control: HashMap::new(),
            },
            black: PlayerData {
                eat: HashMap::new(),
                control: HashMap::new(),
            },
            history: vec![],
            pass: None,
        }
    }

    pub fn classic(played: bool) -> Self {
        let mut base = [None; 64];
        let white = [
            Some(Piece::new(true, PieceKind::Rook)),
            Some(Piece::new(true, PieceKind::Knight)),
            Some(Piece::new(true, PieceKind::Bishop)),
            Some(Piece::new(true, PieceKind::Queen)),
            Some(Piece::new(true, PieceKind::King)),
            Some(Piece::new(true, PieceKind::Bishop)),
            Some(Piece::new(true, PieceKind::Knight)),
            Some(Piece::new(true, PieceKind::Rook)),
        ];
        let black = [
            Some(Piece::new(false, PieceKind::Rook)),
            Some(Piece::new(false, PieceKind::Knight)),
            Some(Piece::new(false, PieceKind::Bishop)),
            Some(Piece::new(false, PieceKind::Queen)),
            Some(Piece::new(false, PieceKind::King)),
            Some(Piece::new(false, PieceKind::Bishop)),
            Some(Piece::new(false, PieceKind::Knight)),
            Some(Piece::new(false, PieceKind::Rook)),
        ];
        if !played {
            base[0..8].copy_from_slice(&white);
            base[8..16].copy_from_slice(&[Some(Piece::new(true, PieceKind::Pawn)); 8]);
            base[48..56].copy_from_slice(&[Some(Piece::new(false, PieceKind::Pawn)); 8]);
            base[56..64].copy_from_slice(&black);
        } else {
            base[56..64].copy_from_slice(&white);
            base[48..56].copy_from_slice(&[Some(Piece::new(true, PieceKind::Pawn)); 8]);
            base[8..16].copy_from_slice(&[Some(Piece::new(false, PieceKind::Pawn)); 8]);
            base[0..8].copy_from_slice(&black);
        }

        Self {
            board: base,
            played,
            white: PlayerData {
                eat: HashMap::new(),
                control: HashMap::new(),
            },
            black: PlayerData {
                eat: HashMap::new(),
                control: HashMap::new(),
            },
            history: vec![],
            pass: None,
        }
    }

    pub fn place(&mut self, p: Piece, to: &Pos) {
        self.board[(to.i * 8 + to.j) as usize] = Some(p);
    }

    // get all possible move without check the king rule
}

// controle
impl Engin {
    pub fn update_control(&mut self) {
        self.white.control.clear();
        self.black.control.clear();

        for i in 0..8 {
            for j in 0..8 {
                self.add_control(&Pos::new_c(i, j));
            }
        }
    }

    fn add_control(&mut self, from: &Pos) {
        match self.board[from.index()] {
            Some(p) => {
                let base = self.get_theoric_moves(from, true);
                for pos in base {
                    if p.color {
                        *self.white.control.entry(pos).or_insert(0) += 1
                    } else {
                        *self.black.control.entry(pos).or_insert(0) += 1
                    }
                }
            }
            None => (),
        }
    }

    fn remove_control(&mut self, from: &Pos) {
        match self.board[from.index()] {
            Some(p) => {
                let base = self.get_theoric_moves(from, true);
                let mut remove;
                let map = if p.color {
                    &mut self.white
                } else {
                    &mut self.black
                };
                for pos in base {
                    remove = false;
                    if let Some(value) = map.control.get_mut(&pos) {
                        if *value > 0 {
                            *value -= 1;
                            remove = *value == 0;
                        }
                    }
                    if remove {
                        map.control.remove(&pos);
                    }
                }
            }
            None => (),
        }
    }
}

// movement

impl Engin {
    fn get_theoric_moves(&self, from: &Pos, control: bool) -> Vec<Pos> {
        let mut base = vec![];
        let index = (from.i * 8 + from.j) as usize;
        let cell = self.board[index];
        match cell {
            Some(Piece { color, kind }) => match kind {
                PieceKind::King => Self::king(&self, &from, &mut base, color, control),
                PieceKind::Queen => Self::move_line(
                    &self,
                    &from,
                    &mut base,
                    color,
                    &vec![
                        [1, 0],
                        [-1, 0],
                        [0, 1],
                        [0, -1],
                        [1, 1],
                        [1, -1],
                        [-1, 1],
                        [-1, -1],
                    ],
                ),
                PieceKind::Rook => Self::move_line(
                    &self,
                    &from,
                    &mut base,
                    color,
                    &vec![[1, 0], [-1, 0], [0, 1], [0, -1]],
                ),
                PieceKind::Bishop => Self::move_line(
                    &self,
                    &from,
                    &mut base,
                    color,
                    &vec![[1, 1], [1, -1], [-1, 1], [-1, -1]],
                ),
                PieceKind::Knight => Self::patern(
                    &self,
                    &from,
                    &mut base,
                    color,
                    &vec![
                        [2, 1],
                        [2, -1],
                        [-2, 1],
                        [-2, -1],
                        [1, 2],
                        [1, -2],
                        [-1, 2],
                        [-1, -2],
                    ],
                ),
                PieceKind::Pawn => Self::pawn(&self, &from, &mut base, color, control),
            },
            None => (),
        }
        base
    }

    pub fn get_moves(&self, from: &Pos) -> Vec<Pos> {
        let mut base = Self::get_theoric_moves(self, from, false);

        base
    }

    // rook bishop queen
    fn move_line(&self, from: &Pos, base: &mut Vec<Pos>, color_piece: bool, dir: &Vec<[i8; 2]>) {
        for d in dir {
            let mut i: i8 = from.i + d[0];
            let mut j: i8 = from.j + d[1];
            while i < 8 && j < 8 && i >= 0 && j >= 0 {
                match self.board[(i * 8 + j) as usize] {
                    Some(Piece { color, kind: _ }) => {
                        if color ^ color_piece {
                            // adverse color
                            base.push(Pos::new_c(i, j));
                            break;
                        }
                    }
                    None => base.push(Pos::new_c(i, j)),
                }

                i += d[0];
                j += d[1];
            }
        }
    }

    // knight
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
                        // adverse color
                        base.push(Pos::new_c(i, j));
                        continue;
                    }
                }
                None => base.push(Pos::new_c(i, j)),
            }
        }
    }

    // pawn
    fn pawn(&self, from: &Pos, base: &mut Vec<Pos>, color_piece: bool, control: bool) {
        // la couleur importe pas, si tu joue un pion il avance vers 0 le reste avance vers 7
        // si tu joue les blanc les blanc il avance vers 0
        // si tu joue les noir les noir avance vers 0

        let dir = if self.played { -1 } else { 1 };

        if let Some(p) = self.pass {
            if (from.j - p.j.abs() == 1) && (from.i == p.i - dir) {
                if self.board[p.index()].is_none() {
                    base.push(p);
                }
            }
        }

        if from.j > 0 {
            // eat to left
            match self.board[from.indexa(dir, -1)] {
                Some(p) => {
                    if p.color ^ color_piece {
                        base.push(Pos::new_c(from.i + dir, from.j - 1));
                    }
                }
                None => {
                    if control {
                        base.push(Pos::new_c(from.i + dir, from.j - 1));
                    }
                }
            }
        }
        if from.j < 7 {
            // eat to right
            match self.board[from.indexa(dir, 1)] {
                Some(p) => {
                    if p.color ^ color_piece {
                        base.push(Pos::new_c(from.i + dir, from.j + 1));
                    }
                }
                None => {
                    if control {
                        base.push(Pos::new_c(from.i + dir, from.j + 1));
                    }
                }
            }
        }
        if !(self.played ^ color_piece) {
            if from.i == 6 {
                if self.board[from.indexa(-1, 0)].is_none() {
                    base.push(Pos::new_c(from.i - 1, from.j));
                    if self.board[from.indexa(-2, 0)].is_none() {
                        base.push(Pos::new_c(from.i - 2, from.j));
                    }
                }
            }
        } else {
            if from.i == 1 {
                if self.board[from.indexa(1, 0)].is_none() {
                    if self.board[from.indexa(2, 0)].is_none() {
                        base.push(Pos::new_c(from.i + 2, from.j));
                    }
                    base.push(Pos::new_c(from.i + 1, from.j));
                }
            }
        }
    }

    // king
    fn king(&self, from: &Pos, base: &mut Vec<Pos>, color_piece: bool, control: bool) {
        // manque juste le rock
        for y in from.i - 1..from.i + 2 {
            for x in from.j - 1..from.j + 2 {
                if y >= 0 && x >= 0 && y < 8 && x < 8 {
                    if !(x != from.j && y != from.i) {
                        let look = Pos::new_c(y, x);
                        match self.board[(y * 8 + x) as usize] {
                            Some(Piece { color, kind: _ }) => {
                                if color ^ color_piece {
                                    base.push(look);
                                }
                            }
                            None => match color_piece {
                                true => {
                                    if !self.black.control.contains_key(&look) || control {
                                        base.push(look);
                                    }
                                }
                                false => {
                                    if !self.white.control.contains_key(&look) || control {
                                        base.push(look);
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

impl fmt::Display for Engin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Board: \n")?;
        for i in 0..8 {
            write!(f, "{}", 8 - i)?;
            for j in 0..8 {
                let index = i * 8 + j;
                match self.board[index] {
                    Some(p) => write!(f, " {}", p.icon())?,
                    None => write!(f, "  .")?,
                }
            }
            writeln!(f, "")?;
        }
        for i in 0..8 {
            write!(f, "  {}", (b'a' + i) as char)?;
        }
        Ok(())
    }
}
